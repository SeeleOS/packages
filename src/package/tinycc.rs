use crate::build::CC;
use crate::command::{CommandSpec, make, run};
use crate::fs_utils::remove_if_exists;
use crate::install::install_file;
use crate::make_package;

make_package!(
    TinyCc,
    "tinycc",
    git_url = "https://github.com/TinyCC/tinycc.git",
    git_commit = "fada98b",
    package_impl = {
        fn install_name(&self) -> &'static str {
            "tcc"
        }

        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            run(CommandSpec::new("./configure")
                .arg("--prefix=/")
                .cwd(&self.calc_paths(ctx).src))?;
            Ok(())
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            build_tcc_tools(&paths)?;

            let full_target = paths.build.join("tcc");
            remove_if_exists(&full_target)?;
            run(make()
                .arg("-f")
                .arg("Makefile")
                .arg(format!("CC={}", CC))
                .arg("tcc")
                .cwd(&paths.src))?;

            std::fs::rename(paths.src.join("tcc"), &full_target)?;
            Ok(())
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            let source = paths.build.join("tcc");
            let target = ctx.install_dir.join("tcc");
            install_file(self, &source, &target)
        }
    }
);

fn build_tcc_tools(paths: &crate::types::PackagePaths) -> crate::types::Result<()> {
    let c2str = paths.src.join("c2str.exe");
    if !c2str.is_file() {
        println!("Building host tool c2str.exe...");
        run(CommandSpec::new("gcc")
            .arg("-DC2STR")
            .arg("conftest.c")
            .arg("-o")
            .arg("c2str.exe")
            .cwd(&paths.src))?;
    }
    let tccdefs = paths.src.join("tccdefs_.h");
    if !tccdefs.is_file() {
        println!("Generating tccdefs_.h...");
        run(CommandSpec::new("./c2str.exe")
            .arg("include/tccdefs.h")
            .arg("tccdefs_.h")
            .cwd(&paths.src))?;
    }
    Ok(())
}
