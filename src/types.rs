use std::path::PathBuf;

use crate::fs_utils::ensure_dir;
use crate::layout::{BINDIR, INCLUDEDIR, LIB_BINARY_DIR, LIBDIR, relative_dir};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Install,
    Clean,
    RebuildOnly,
}

impl Action {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "install" => Some(Action::Install),
            "clean" => Some(Action::Clean),
            "rebuild-only" => Some(Action::RebuildOnly),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub packages_root: PathBuf,
    pub pkg_specific_root: PathBuf,
    pub staging_sysroot_dir: PathBuf,
    pub real_sysroot_dir: PathBuf,
    pub install_dir: PathBuf,
    pub include_root_dir: PathBuf,
    pub include_c_dir: PathBuf,
    pub lib_binary_dir: PathBuf,
    pub lib_dir: PathBuf,
    pub rebuild: bool,
    pub ignore_deps: bool,
}

impl Context {
    pub fn discover(rebuild: bool, ignore_deps: bool) -> Result<Self> {
        let cwd = std::env::current_dir()?;
        let packages_root = if cwd.join("README.md").is_file() && cwd.join("pkg-specific").is_dir()
        {
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
        let pkg_specific_root = packages_root.join("pkg-specific");
        let staging_sysroot_dir = packages_root.join("work/sysroot-stage");
        let include_root_dir = staging_sysroot_dir.join(relative_dir(INCLUDEDIR));
        Ok(Self {
            pkg_specific_root,
            staging_sysroot_dir: staging_sysroot_dir.clone(),
            real_sysroot_dir: base.join("sysroot"),
            install_dir: staging_sysroot_dir.join(relative_dir(BINDIR)),
            include_root_dir: include_root_dir.clone(),
            include_c_dir: include_root_dir.join("c"),
            lib_binary_dir: staging_sysroot_dir.join(relative_dir(LIB_BINARY_DIR)),
            lib_dir: staging_sysroot_dir.join(relative_dir(LIBDIR)),
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
    pub patch: PathBuf,
    pub build: PathBuf,
    pub pkg_specific: PathBuf,
}

impl PackagePaths {
    pub fn ensure(&self) -> Result<()> {
        ensure_dir(&self.root)?;
        ensure_dir(&self.src)?;
        ensure_dir(&self.stamp)?;
        ensure_dir(&self.build)?;

        Ok(())
    }
}
