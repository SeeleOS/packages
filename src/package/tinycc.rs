use crate::command::{CommandSpec, run};
use crate::fs_utils::{copy_file_with_sudo, ensure_dir, remove_if_exists, touch, verify_same_size};
use crate::traits::{GitCloneFetch, Package};
use crate::types::{Context, Result};

pub struct TinyCc;

impl Package for TinyCc {
    fn name(&self) -> &'static str {
        "tinycc"
    }

    fn install_name(&self) -> &'static str {
        "tcc"
    }

    fn fetch(&self, ctx: &Context) -> Result<()> {
        <Self as GitCloneFetch>::fetch(self, ctx)
    }

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.paths(ctx);
        self.patch(ctx)?;
        println!("[packages][tinycc] configuring...");
        ensure_dir(&paths.stamp)?;
        run(CommandSpec::new("./configure").arg("--prefix=/").cwd(&paths.src))?;
        touch(&paths.stamp.join("configure"))?;
        Ok(())
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        let paths = self.paths(ctx);
        self.configure(ctx)?;
        println!("[packages][tinycc] building relibc...");
        run(CommandSpec::new("make").arg("-C").arg(&ctx.relibc_root).arg("all"))?;
        println!("[packages][tinycc] building TinyCC...");
        ensure_dir(&paths.build)?;

        let c2str = paths.src.join("c2str.exe");
        if !c2str.is_file() {
            println!("Building host tool c2str.exe...");
            run(
                CommandSpec::new("gcc")
                    .arg("-DC2STR")
                    .arg("conftest.c")
                    .arg("-o")
                    .arg("c2str.exe")
                    .cwd(&paths.src),
            )?;
        }
        let tccdefs = paths.src.join("tccdefs_.h");
        if !tccdefs.is_file() {
            println!("Generating tccdefs_.h...");
            run(
                CommandSpec::new("./c2str.exe")
                    .arg("include/tccdefs.h")
                    .arg("tccdefs_.h")
                    .cwd(&paths.src),
            )?;
        }
        let full_target = paths.build.join("tcc");
        remove_if_exists(&full_target)?;
        run(
            CommandSpec::new("make")
                .arg("-f")
                .arg("Makefile")
                .arg("CC=x86_64-elf-gcc")
                .arg("tcc")
                .cwd(&paths.src),
        )?;
        std::fs::rename(paths.src.join("tcc"), &full_target)?;
        let _ = run(CommandSpec::new("readelf").arg("-h").arg(&full_target));
        Ok(())
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.paths(ctx);
        self.build(ctx)?;
        let source = paths.build.join("tcc");
        let target = ctx.install_dir.join(self.install_name());
        println!("[packages][tinycc] installing {}...", target.display());
        copy_file_with_sudo(&source, &target)?;
        verify_same_size(&source, &target)?;
        println!("[packages][tinycc][OK]: installation verified.");
        Ok(())
    }
}

impl GitCloneFetch for TinyCc {
    fn git_url(&self) -> &'static str {
        "https://github.com/TinyCC/tinycc.git"
    }

    fn git_commit(&self) -> &'static str {
        "fada98b"
    }
}
