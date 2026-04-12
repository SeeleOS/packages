use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use crate::build::{CC, build_make_in};
use crate::command::{CommandSpec, capture, make, run};
use crate::fs_utils::{create_symlink_force, remove_if_exists};
use crate::install::install_file;
use crate::make_package;

make_package!(
    Busybox,
    "busybox",
    tarball_url = "https://mirrors.aliyun.com/slackware/slackware64-current/source/installer/sources/busybox/busybox-1.37.0.tar.bz2",
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            load_config(ctx, &paths)?;
            run(CommandSpec::new("sh").arg("-c").arg(format!(
                "yes \"\" | make -C '{}' O='{}' HOSTCC='gcc' oldconfig >/dev/null",
                paths.src.display(),
                paths.build.display()
            )))?;
            Ok(())
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            build_make_in(
                &paths.src,
                Vec::new(),
                vec![
                    format!("O={}", paths.build.display()),
                    "HOSTCC=gcc".to_string(),
                    format!("CC={}", CC),
                    "busybox".to_string(),
                ],
            )
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            let source = paths.build.join("busybox");
            let busybox_bin = ctx.install_dir.join("busybox");
            let applets = busybox_applets(&paths)?;

            install_file(self, &source, &busybox_bin)?;
            for applet in &applets {
                install_busybox_symlink(&ctx.install_dir, applet)?;
            }

            let ls_link = ctx.install_dir.join("ls");
            if !busybox_bin.is_file() {
                return Err(format!("{} was not installed", busybox_bin.display()).into());
            }
            if !std::fs::symlink_metadata(&ls_link)?
                .file_type()
                .is_symlink()
            {
                return Err(format!("{} is not a symlink", ls_link.display()).into());
            }
            Ok(())
        }
    }
);

fn load_config(
    ctx: &crate::types::Context,
    paths: &crate::types::PackagePaths,
) -> crate::types::Result<()> {
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

fn busybox_applets(paths: &crate::types::PackagePaths) -> crate::types::Result<Vec<PathBuf>> {
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

fn applet_path(dir: &str, name: &str) -> crate::types::Result<PathBuf> {
    match dir {
        "BB_DIR_BIN" | "BB_DIR_SBIN" | "BB_DIR_USR_BIN" | "BB_DIR_USR_SBIN" | "BB_DIR_ROOT" => {
            Ok(PathBuf::from(name))
        }
        other => Err(format!("unknown busybox applet dir: {other}").into()),
    }
}

fn install_busybox_symlink(install_dir: &PathBuf, link_rel: &PathBuf) -> crate::types::Result<()> {
    let link = install_dir.join(link_rel);
    create_symlink_force(std::path::Path::new("busybox"), &link)?;
    Ok(())
}
