use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Install,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub packages_root: PathBuf,
    pub relibc_root: PathBuf,
    pub relibc_path: PathBuf,
    pub install_dir: PathBuf,
}

impl Context {
    pub fn discover() -> Result<Self> {
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
            relibc_root: base.join("relibc-seele"),
            relibc_path: base.join("relibc-seele/target/x86_64-seele/release"),
            install_dir: base.join("sysroot/programs"),
            packages_root,
        })
    }
}

#[derive(Clone, Debug)]
pub struct RecipePaths {
    pub root: PathBuf,
    pub src: PathBuf,
    pub stamp: PathBuf,
    pub patches: PathBuf,
    pub build: PathBuf,
}
