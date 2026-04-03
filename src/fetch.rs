use std::fs;

use crate::command::{CommandSpec, run};
use crate::fs_utils::{download_file, ensure_dir, remove_path_if_exists};
use crate::r#trait::Package;
use crate::types::{Context, Result};

#[macro_export]
macro_rules! fetch_wrap {
    ($type: ty) => {
        fn fetch(&self, ctx: &Context) -> Result<()> {
            <Self as $type>::fetch(self, ctx)
        }
    };
}

pub trait TarballFetch: Package {
    fn tarball_url(&self) -> Vec<&'static str>;

    fn tarball_name(&self) -> &'static str {
        self.tarball_url()
            .first()
            .and_then(|url| url.rsplit('/').next())
            .unwrap_or(self.name())
    }

    fn fetch(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        ensure_dir(&paths.root)?;
        ensure_dir(&paths.stamp)?;

        let tarball = paths.root.join(self.tarball_name());
        if paths.src.exists() {
            fs::remove_dir_all(&paths.src)?;
        }
        remove_path_if_exists(&tarball)?;
        download_file(&tarball, &self.tarball_url(), &paths.root)?;

        ensure_dir(&paths.src)?;
        run(CommandSpec::new("tar")
            .arg("-xf")
            .arg(&tarball)
            .arg("--strip-components=1")
            .arg("-C")
            .arg(&paths.src))?;
        Ok(())
    }
}

pub trait GitCloneFetch: Package {
    fn git_url(&self) -> &'static str;
    fn git_commit(&self) -> &'static str;

    fn fetch(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        ensure_dir(&paths.root)?;
        ensure_dir(&paths.stamp)?;
        if paths.src.join(".git").is_dir() {
            return Ok(());
        }
        if paths.src.exists() {
            fs::remove_dir_all(&paths.src)?;
        }
        run(CommandSpec::new("git")
            .arg("clone")
            .arg(self.git_url())
            .arg(&paths.src))?;
        run(CommandSpec::new("git")
            .arg("checkout")
            .arg(self.git_commit())
            .cwd(&paths.src))?;
        Ok(())
    }
}
