use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use crate::build::CC;
use crate::command::{CommandSpec, capture, make, run};
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

        run(make()
            .arg("-C")
            .arg(&paths.src)
            .arg(format!("O={}", paths.build.display()))
            .arg("HOSTCC=gcc")
            .arg(format!("CC={}", CC))
            .arg("busybox"))?;

        Ok(())
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let source = paths.build.join("busybox");
        let busybox_bin = ctx.install_dir.join("busybox");
        let applets = busybox_applets(&paths)?;

        println!(
            "[packages][busybox] installing {}...",
            busybox_bin.display()
        );
        copy_file_with_sudo(&source, &busybox_bin)?;
        verify_same_size(&source, &busybox_bin)?;
        for applet in &applets {
            install_busybox_symlink(&ctx.install_dir, applet)?;
        }
        run(CommandSpec::new("sync"))?;

        let busybox_bin = ctx.install_dir.join("busybox");
        let ls_link = ctx.install_dir.join("ls");
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
    run(make()
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
        entries.insert(applet_path(dir, name)?);
    }

    Ok(entries.into_iter().collect())
}

fn applet_path(dir: &str, name: &str) -> Result<PathBuf> {
    match dir {
        "BB_DIR_BIN" | "BB_DIR_SBIN" | "BB_DIR_USR_BIN" | "BB_DIR_USR_SBIN" | "BB_DIR_ROOT" => {
            Ok(PathBuf::from(name))
        }
        other => return Err(format!("unknown busybox applet dir: {other}").into()),
    }
}

fn install_busybox_symlink(install_dir: &PathBuf, link_rel: &PathBuf) -> Result<()> {
    let link = install_dir.join(link_rel);
    run(CommandSpec::new("sudo")
        .arg("ln")
        .arg("-sfn")
        .arg("busybox")
        .arg(&link))?;
    Ok(())
}
