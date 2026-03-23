use std::fs;

use crate::command::{CommandSpec, run};
use crate::fs_utils::{download_file, ensure_dir, remove_path_if_exists};
use crate::trace::{package, package_detail};
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
        package(self.name(), "fetching sources");
        ensure_dir(&paths.root)?;
        ensure_dir(&paths.stamp)?;

        let tarball = paths.root.join(self.tarball_name());
        if paths.src.exists() {
            package_detail(
                self.name(),
                format!("removing previous source tree {}", paths.src.display()),
            );
            fs::remove_dir_all(&paths.src)?;
        }
        package_detail(
            self.name(),
            format!("clearing previous tarball {}", tarball.display()),
        );
        remove_path_if_exists(&tarball)?;
        package_detail(
            self.name(),
            format!("candidate URLs: {}", self.tarball_url().join(", ")),
        );
        download_file(&tarball, &self.tarball_url(), &paths.root)?;

        ensure_dir(&paths.src)?;
        package_detail(
            self.name(),
            format!(
                "extracting {} into {}",
                tarball.display(),
                paths.src.display()
            ),
        );
        run(CommandSpec::new("tar")
            .arg("-xf")
            .arg(&tarball)
            .arg("--strip-components=1")
            .arg("-C")
            .arg(&paths.src))?;
        package(self.name(), "source fetch and extraction complete");
        Ok(())
    }
}

pub trait GitCloneFetch: Package {
    fn git_url(&self) -> &'static str;
    fn git_commit(&self) -> &'static str;

    fn fetch(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        package(self.name(), "fetching sources");
        ensure_dir(&paths.root)?;
        ensure_dir(&paths.stamp)?;
        if paths.src.join(".git").is_dir() {
            package_detail(
                self.name(),
                format!("reusing existing clone at {}", paths.src.display()),
            );
            return Ok(());
        }
        if paths.src.exists() {
            package_detail(
                self.name(),
                format!("removing non-git source tree {}", paths.src.display()),
            );
            fs::remove_dir_all(&paths.src)?;
        }
        package_detail(
            self.name(),
            format!("cloning {} at {}", self.git_url(), self.git_commit()),
        );
        run(CommandSpec::new("git")
            .arg("clone")
            .arg(self.git_url())
            .arg(&paths.src))?;
        run(CommandSpec::new("git")
            .arg("checkout")
            .arg(self.git_commit())
            .cwd(&paths.src))?;
        package(self.name(), "git fetch complete");
        Ok(())
    }
}
