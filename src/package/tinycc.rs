use crate::build::{CC, build_relibc};
use crate::command::{CommandSpec, run};
use crate::fetch::GitCloneFetch;
use crate::fs_utils::{ensure_dir, remove_if_exists};
use crate::install::Install;
use crate::r#trait::Package;
use crate::types::{Context, PackagePaths, Result};
use crate::{fetch_wrap, install_wrap};

pub struct TinyCc;

impl Package for TinyCc {
    fn name(&self) -> &'static str {
        "tinycc"
    }

    fn install_name(&self) -> &'static str {
        "tcc"
    }

    fetch_wrap!(GitCloneFetch);
    install_wrap!();

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        run(CommandSpec::new("./configure")
            .arg("--prefix=/")
            .cwd(&paths.src))?;
        Ok(())
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);

        build_tcc_tools(&paths)?;

        let full_target = paths.build.join("tcc");
        remove_if_exists(&full_target)?;
        run(CommandSpec::new("make")
            .arg("-f")
            .arg("Makefile")
            .arg(format!("CC={}", CC))
            .arg("tcc")
            .cwd(&paths.src))?;

        std::fs::rename(paths.src.join("tcc"), &full_target)?;
        let _ = run(CommandSpec::new("readelf").arg("-h").arg(&full_target));
        Ok(())
    }
}

fn build_tcc_tools(paths: &PackagePaths) -> Result<()> {
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

impl GitCloneFetch for TinyCc {
    fn git_url(&self) -> &'static str {
        "https://github.com/TinyCC/tinycc.git"
    }

    fn git_commit(&self) -> &'static str {
        "fada98b"
    }
}

impl Install for TinyCc {
    fn binary_name(&self) -> &'static str {
        "tcc"
    }
}
