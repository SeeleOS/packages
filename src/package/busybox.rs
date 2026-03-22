use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::build::CC;
use crate::command::{CommandSpec, capture, run};
use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::fs_utils::{copy_file_with_sudo, ensure_dir, remove_if_exists, verify_same_size};
use crate::r#trait::Package;
use crate::types::{Context, PackagePaths, Result};

pub struct Busybox;

impl Package for Busybox {
    fn name(&self) -> &'static str {
        "busybox"
    }

    fetch_wrap!(TarballFetch);

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        println!("[packages][busybox] configuring...");
        ensure_dir(&paths.build)?;

        load_config(ctx, &paths)?;

        run(CommandSpec::new("sh").arg("-c").arg(format!(
            "yes \"\" | make -C '{}' O='{}' HOSTCC='gcc' oldconfig >/dev/null",
            paths.src.display(),
            paths.build.display()
        )))?;
        Ok(())
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);

        run(CommandSpec::new("make")
            .arg("-C")
            .arg(&paths.src)
            .arg(format!("O={}", paths.build.display()))
            .arg("HOSTCC=gcc")
            .arg(format!("CC={}", CC))
            .arg("busybox"))?;
        let _ = run(CommandSpec::new("readelf")
            .arg("-h")
            .arg(paths.build.join("busybox")));
        Ok(())
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let source = paths.build.join("busybox");
        let busybox_bin = ctx.install_dir.join("bin/busybox");

        println!("[packages][busybox] installing {}...", busybox_bin.display());
        copy_file_with_sudo(&source, &busybox_bin)?;
        verify_same_size(&source, &busybox_bin)?;

        let old_links = collect_busybox_symlinks(&ctx.install_dir, &busybox_bin)?;
        for link in old_links {
            run(CommandSpec::new("sudo").arg("rm").arg("-f").arg(&link))?;
        }

        let applets = busybox_applets(&paths)?;
        for applet in &applets {
            install_busybox_symlink(&ctx.install_dir, applet)?;
        }
        run(CommandSpec::new("sync"))?;

        let busybox_bin = ctx.install_dir.join("bin/busybox");
        let ls_link = ctx.install_dir.join("bin/ls");
        if !busybox_bin.is_file() {
            return Err(format!("{} was not installed", busybox_bin.display()).into());
        }
        if capture(CommandSpec::new("test").arg("-L").arg(&ls_link)).is_err() {
            return Err(format!("{} is not a symlink", ls_link.display()).into());
        }
        println!(
            "[packages][busybox][OK]: busybox and {} symlink applets installed.",
            applets.len()
        );
        Ok(())
    }
}

// Loads the custom seele.config into busybox
fn load_config(ctx: &Context, paths: &PackagePaths) -> Result<()> {
    let config_in = ctx.packages_root.join("busybox/seele.config");
    let build_config = paths.build.join(".config");
    remove_if_exists(&build_config)?;
    run(CommandSpec::new("make")
        .arg("-C")
        .arg(&paths.src)
        .arg(format!("O={}", paths.build.display()))
        .arg("HOSTCC=gcc")
        .arg("allnoconfig"))?;

    let mut config = fs::read_to_string(&build_config)?;
    for line in fs::read_to_string(config_in)?.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            remove_config_key(&mut config, key);
            if value == "n" {
                config.push_str(&format!("# {key} is not set\n"));
            } else {
                config.push_str(&format!("{key}={value}\n"));
            }
        }
    }

    fs::write(&build_config, config)?;

    Ok(())
}

impl TarballFetch for Busybox {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec![
            "https://mirrors.aliyun.com/slackware/slackware64-current/source/installer/sources/busybox/busybox-1.37.0.tar.bz2",
            "https://busybox.net/downloads/busybox-1.37.0.tar.bz2",
        ]
    }
}

fn remove_config_key(config: &mut String, key: &str) {
    let key_eq = format!("{key}=");
    let key_not_set = format!("# {key} is not set");
    let kept = config
        .lines()
        .filter(|line| !line.starts_with(&key_eq) && *line != key_not_set)
        .collect::<Vec<_>>()
        .join("\n");
    *config = if kept.is_empty() {
        String::new()
    } else {
        format!("{kept}\n")
    };
}

fn busybox_applets(paths: &PackagePaths) -> Result<Vec<PathBuf>> {
    let autoconf = paths.build.join("include/autoconf.h");
    let applets = paths.build.join("include/applets.h");
    let install_no_usr =
        fs::read_to_string(&autoconf)?.contains("#define ENABLE_INSTALL_NO_USR 1");
    let output = capture(
        CommandSpec::new("gcc")
            .arg("-E")
            .arg("-DMAKE_LINKS")
            .arg("-include")
            .arg(&autoconf)
            .arg(&applets),
    )?;

    let mut entries = BTreeSet::new();
    for line in output.lines() {
        let Some(rest) = line.strip_prefix("LINK ") else {
            continue;
        };
        let mut parts = rest.split_whitespace();
        let Some(dir) = parts.next() else {
            continue;
        };
        let Some(name) = parts.next() else {
            continue;
        };
        if name == "busybox" {
            continue;
        }
        entries.insert(applet_path(dir, name, install_no_usr)?);
    }

    Ok(entries.into_iter().collect())
}

fn applet_path(dir: &str, name: &str, install_no_usr: bool) -> Result<PathBuf> {
    let mut path = match dir {
        "BB_DIR_BIN" => PathBuf::from("bin"),
        "BB_DIR_SBIN" => PathBuf::from("sbin"),
        "BB_DIR_USR_BIN" => {
            if install_no_usr {
                PathBuf::from("bin")
            } else {
                PathBuf::from("usr/bin")
            }
        }
        "BB_DIR_USR_SBIN" => {
            if install_no_usr {
                PathBuf::from("sbin")
            } else {
                PathBuf::from("usr/sbin")
            }
        }
        "BB_DIR_ROOT" => PathBuf::new(),
        other => return Err(format!("unknown busybox applet dir: {other}").into()),
    };
    path.push(name);
    Ok(path)
}

fn install_busybox_symlink(install_dir: &Path, link_rel: &Path) -> Result<()> {
    let link = install_dir.join(link_rel);
    let parent = link.parent().ok_or("busybox applet install target has no parent")?;
    let parent_rel = parent.strip_prefix(install_dir)?;
    let target = relative_path(parent_rel, Path::new("bin/busybox"));

    run(CommandSpec::new("sudo").arg("mkdir").arg("-p").arg(parent))?;
    run(CommandSpec::new("sudo")
        .arg("ln")
        .arg("-sfn")
        .arg(&target)
        .arg(&link))?;
    Ok(())
}

fn collect_busybox_symlinks(root: &Path, busybox_bin: &Path) -> Result<Vec<PathBuf>> {
    let busybox_bin = fs::canonicalize(busybox_bin)?;
    let mut links = Vec::new();
    collect_busybox_symlinks_inner(root, &busybox_bin, &mut links)?;
    links.sort();
    Ok(links)
}

fn collect_busybox_symlinks_inner(
    dir: &Path,
    busybox_bin: &Path,
    links: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_busybox_symlinks_inner(&path, busybox_bin, links)?;
            continue;
        }
        if !file_type.is_symlink() {
            continue;
        }

        let target = fs::read_link(&path)?;
        let resolved = if target.is_absolute() {
            target
        } else {
            path.parent()
                .ok_or("busybox symlink has no parent")?
                .join(target)
        };
        if fs::canonicalize(&resolved).ok().as_deref() == Some(busybox_bin) {
            links.push(path);
        }
    }
    Ok(())
}

fn relative_path(from: &Path, to: &Path) -> PathBuf {
    let from_parts = from.iter().collect::<Vec<_>>();
    let to_parts = to.iter().collect::<Vec<_>>();
    let common_len = from_parts
        .iter()
        .zip(&to_parts)
        .take_while(|(a, b)| a == b)
        .count();

    let mut path = PathBuf::new();
    for _ in common_len..from_parts.len() {
        path.push("..");
    }
    for part in &to_parts[common_len..] {
        path.push(part);
    }
    path
}
