use std::path::PathBuf;

use crate::fs_utils::ensure_dir;
use crate::trace::detail;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Install,
    Clean,
}

impl Action {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "install" => Some(Action::Install),
            "clean" => Some(Action::Clean),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub packages_root: PathBuf,
    pub relibc_root: PathBuf,
    pub relibc_path: PathBuf,
    pub install_dir: PathBuf,
    pub system_include_dir: PathBuf,
    pub system_lib_dir: PathBuf,
}

impl Context {
    pub fn discover() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        detail(format!(
            "discovering package context from cwd={}",
            cwd.display()
        ));
        let packages_root = if cwd.join("README.md").is_file() && cwd.join("bash").is_dir() {
            detail("detected packages root from current directory");
            cwd
        } else if cwd.join("packages").join("README.md").is_file() {
            detail("detected packages root from nested `packages/` directory");
            cwd.join("packages")
        } else {
            return Err(
                "could not locate packages directory from current working directory".into(),
            );
        };
        let base = packages_root
            .parent()
            .ok_or("packages directory has no parent")?
            .to_path_buf();
        detail(format!("using project base directory {}", base.display()));
        Ok(Self {
            relibc_root: base.join("relibc-seele"),
            relibc_path: base.join("relibc-seele/target/x86_64-seele/release"),
            install_dir: base.join("sysroot/programs"),
            system_include_dir: base.join("sysroot/misc/libs/system_include"),
            system_lib_dir: base.join("sysroot/misc/libs/system_lib"),
            packages_root,
        })
    }
}

#[derive(Clone, Debug)]
pub struct PackagePaths {
    pub root: PathBuf,
    pub src: PathBuf,
    pub stamp: PathBuf,
    pub patches: PathBuf,
    pub build: PathBuf,
}

impl PackagePaths {
    pub fn ensure(&self) -> Result<()> {
        detail(format!(
            "ensuring package workspace {}",
            self.root.display()
        ));
        ensure_dir(&self.root)?;
        ensure_dir(&self.src)?;
        ensure_dir(&self.stamp)?;
        ensure_dir(&self.patches)?;
        ensure_dir(&self.build)?;

        Ok(())
    }
}
