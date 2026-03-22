use std::fs;

use crate::command::{CommandSpec, capture, run};
use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::fs_utils::{ensure_dir, remove_if_exists};
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct Busybox;

impl Package for Busybox {
    fn name(&self) -> &'static str {
        "busybox"
    }

    fetch_wrap!(TarballFetch);

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        self.patch(ctx)?;
        println!("[packages][busybox] configuring...");
        ensure_dir(&paths.build)?;

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

        let relibc_include = ctx.relibc_root.join("target/x86_64-seele/include");
        config.push_str(&format!(
            "CONFIG_EXTRA_CFLAGS=\"-Wall -O2 -ffreestanding -mno-sse -mno-red-zone -fno-stack-protector -fno-builtin -fno-pie -no-pie -nostdinc -I{} -D__seele__ -D_GNU_SOURCE\"\n",
            relibc_include.display()
        ));
        config.push_str(&format!(
            "CONFIG_EXTRA_LDFLAGS=\"-static -nostdlib -L{}\"\n",
            ctx.relibc_path.display()
        ));
        config.push_str("CONFIG_EXTRA_LDLIBS=\"-l:libc.a -l:crtn.o\"\n");
        fs::write(&build_config, config)?;

        run(CommandSpec::new("sh").arg("-c").arg(format!(
            "yes \"\" | make -C '{}' O='{}' HOSTCC='gcc' oldconfig >/dev/null",
            paths.src.display(),
            paths.build.display()
        )))?;
        Ok(())
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        self.configure(ctx)?;
        println!("[packages][busybox] checking relibc artifacts...");
        for dep in ["crt0.o", "crti.o", "crtn.o", "libc.a"] {
            if !ctx.relibc_path.join(dep).is_file() {
                return Err(format!("missing relibc artifact: {dep}").into());
            }
        }
        println!("[packages][busybox] building busybox...");
        run(CommandSpec::new("make")
            .arg("-C")
            .arg(&paths.src)
            .arg(format!("O={}", paths.build.display()))
            .arg("HOSTCC=gcc")
            .arg("CC=x86_64-elf-gcc")
            .arg("AR=x86_64-elf-ar")
            .arg("CROSS_COMPILE=x86_64-elf-")
            .arg("CFLAGS_busybox=-l:crt0.o -l:crti.o")
            .arg("busybox"))?;
        let _ = run(CommandSpec::new("readelf")
            .arg("-h")
            .arg(paths.build.join("busybox")));
        Ok(())
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        self.build(ctx)?;
        println!(
            "[packages][busybox] installing busybox symlinks into {}...",
            ctx.install_dir.display()
        );
        ensure_dir(&ctx.install_dir)?;

        run(
            CommandSpec::new("sh")
                .arg("-c")
                .arg(format!(
                    "cd '{}' && HOSTCC='gcc' '{}/applets/busybox.mkll' '{}/include/autoconf.h' '{}/include/applets.h' > busybox.links",
                    paths.build.display(),
                    paths.src.display(),
                    paths.build.display(),
                    paths.src.display()
                )),
        )?;
        let cleanup = run(CommandSpec::new("sudo")
            .arg(paths.src.join("applets/install.sh"))
            .arg(&ctx.install_dir)
            .arg("--cleanup")
            .cwd(&paths.build));
        if cleanup.is_err() {
            println!("[packages][busybox] cleanup reported an error and was ignored");
        }
        run(CommandSpec::new("sudo")
            .arg(paths.src.join("applets/install.sh"))
            .arg(&ctx.install_dir)
            .arg("--symlinks")
            .cwd(&paths.build))?;
        run(CommandSpec::new("sync"))?;
        let busybox_bin = ctx.install_dir.join("bin/busybox");
        let ls_link = ctx.install_dir.join("bin/ls");
        if !busybox_bin.is_file() {
            return Err(format!("{} was not installed", busybox_bin.display()).into());
        }
        if capture(CommandSpec::new("test").arg("-L").arg(&ls_link)).is_err() {
            return Err(format!("{} is not a symlink", ls_link.display()).into());
        }
        println!("[packages][busybox][OK]: busybox and symlink applets installed.");
        Ok(())
    }
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
