use crate::command::{CommandSpec, capture, run};
use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::fs_utils::{copy_file_with_sudo, ensure_dir, remove_if_exists, touch, verify_same_size};
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct Bash;

impl Package for Bash {
    fn name(&self) -> &'static str {
        "bash"
    }

    fetch_wrap!(TarballFetch);

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        self.patch(ctx)?;
        println!("[packages][bash] configuring...");
        ensure_dir(&paths.build)?;
        ensure_dir(&paths.stamp)?;

        let relibc_include = ctx.relibc_root.join("target/x86_64-seele/include");
        let cflags = format!(
            "-Wall -O2 -ffreestanding -D__seele__ -mno-red-zone -fno-stack-protector -fno-builtin -fno-pie -no-pie -nostdinc -I{}",
            relibc_include.display()
        );
        let ldflags = format!(
            "-static -nostdlib {}/crt0.o {}/crti.o",
            ctx.relibc_path.display(),
            ctx.relibc_path.display()
        );
        let libs = format!(
            "{}/libc.a {}/crtn.o",
            ctx.relibc_path.display(),
            ctx.relibc_path.display()
        );
        let build_triple = capture(
            CommandSpec::new("sh")
                .arg(paths.src.join("support/config.guess"))
                .cwd(&paths.src),
        )
        .unwrap_or_else(|_| "x86_64-pc-linux-gnu".to_string());

        run(CommandSpec::new("../configure")
            .cwd(&paths.build)
            .env("CC", "x86_64-elf-gcc")
            .env("AR", "x86_64-elf-ar")
            .env("CFLAGS", cflags)
            .env("LDFLAGS", ldflags)
            .env("LIBS", libs)
            .arg("--host=x86_64-unknown-none")
            .arg(format!("--build={}", build_triple.trim()))
            .arg("--prefix=/")
            .arg("--disable-nls")
            .arg("--without-bash-malloc"))?;
        touch(&paths.stamp.join("configure"))?;
        Ok(())
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        self.configure(ctx)?;
        println!("[packages][bash] building relibc...");
        run(CommandSpec::new("make")
            .arg("-C")
            .arg(&ctx.relibc_root)
            .arg("all"))?;
        println!("[packages][bash] building bash...");
        let full_target = paths.build.join("bash");
        for dep in [
            ctx.relibc_path.join("libc.a"),
            ctx.relibc_path.join("crt0.o"),
            ctx.relibc_path.join("crti.o"),
            ctx.relibc_path.join("crtn.o"),
        ] {
            if !full_target.exists()
                || std::fs::metadata(&dep)?.modified()?
                    > std::fs::metadata(&full_target)?.modified()?
            {
                println!("  relibc changed; forcing bash relink...");
                remove_if_exists(&full_target)?;
                break;
            }
        }
        run(CommandSpec::new("make")
            .cwd(&paths.build)
            .env("ADDON_LDFLAGS", "-Wl,--allow-multiple-definition")
            .arg("bash"))?;
        let _ = run(CommandSpec::new("readelf").arg("-h").arg(&full_target));
        Ok(())
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        self.build(ctx)?;
        let source = paths.build.join("bash");
        let target = ctx.install_dir.join("bash");
        println!("[packages][bash] installing {}...", target.display());
        copy_file_with_sudo(&source, &target)?;
        verify_same_size(&source, &target)?;
        println!("[packages][bash][OK]: installation verified.");
        Ok(())
    }
}

impl TarballFetch for Bash {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://ftp.gnu.org/gnu/bash/bash-5.3.tar.gz"]
    }
}
