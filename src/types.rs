use std::path::PathBuf;

use crate::fs_utils::ensure_dir;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Install,
    Deploy,
    Clean,
}

impl Action {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "install" => Some(Action::Install),
            "deploy" => Some(Action::Deploy),
            "clean" => Some(Action::Clean),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub packages_root: PathBuf,
    pub staging_sysroot_dir: PathBuf,
    pub real_sysroot_dir: PathBuf,
    pub relibc_root: PathBuf,
    pub relibc_path: PathBuf,
    pub install_dir: PathBuf,
    pub system_include_dir: PathBuf,
    pub system_lib_dir: PathBuf,
    pub rebuild: bool,
    pub ignore_deps: bool,
}

impl Context {
    pub fn discover(rebuild: bool, ignore_deps: bool) -> Result<Self> {
        let cwd = std::env::current_dir()?;
        let packages_root = if cwd.join("README.md").is_file() && cwd.join("bash").is_dir() {
            cwd
        } else if cwd.join("packages").join("README.md").is_file() {
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
        Ok(Self {
            staging_sysroot_dir: base.join("work/sysroot-stage"),
            real_sysroot_dir: base.join("sysroot"),
            relibc_root: base.join("relibc-seele"),
            relibc_path: base.join("relibc-seele/target/x86_64-seele/release"),
            install_dir: base.join("work/sysroot-stage/programs"),
            system_include_dir: base.join("work/sysroot-stage/libs/include"),
            system_lib_dir: base.join("work/sysroot-stage/libs/lib_binaries"),
            packages_root,
            rebuild,
            ignore_deps,
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
        ensure_dir(&self.root)?;
        ensure_dir(&self.src)?;
        ensure_dir(&self.stamp)?;
        ensure_dir(&self.patches)?;
        ensure_dir(&self.build)?;

        Ok(())
    }
}
