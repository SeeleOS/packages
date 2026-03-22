use std::fs;

use crate::command::{CommandSpec, run};
use crate::fs_utils::{
    download_file, ensure_dir, list_patch_files, remove_if_exists, remove_path_if_exists, touch,
};
use crate::types::{Action, Context, RecipePaths, Result};

#[macro_export]
macro_rules! fetch_wrap {
    ($type: ty) => {
        fn fetch(&self, ctx: &Context) -> Result<()> {
            <Self as $type>::fetch(self, ctx)
        }
    };
}

pub trait Package {
    fn name(&self) -> &'static str;

    fn install_name(&self) -> &'static str {
        self.name()
    }

    fn paths(&self, ctx: &Context) -> RecipePaths {
        let root = ctx.packages_root.join("work").join(self.name());
        RecipePaths {
            src: root.join("src"),
            stamp: root.join(".stamp"),
            patches: ctx.packages_root.join(self.name()).join("patches"),
            build: root.join("src/build"),
            root,
        }
    }

    fn fetch(&self, _ctx: &Context) -> Result<()>;

    fn patch(&self, ctx: &Context) -> Result<()> {
        let paths = self.paths(ctx);
        self.fetch(ctx)?;
        ensure_dir(&paths.stamp)?;
        remove_if_exists(&paths.stamp.join("configure"))?;
        let mut patches = list_patch_files(&paths.patches)?;
        if patches.is_empty() {
            println!("[packages][{}] no patches", self.name());
            touch(&paths.stamp.join("patch"))?;
            return Ok(());
        }
        println!(
            "[packages][{}] applying patches from {}...",
            self.name(),
            paths.patches.display()
        );
        patches.sort();
        for patch in patches {
            println!("  -> {}", patch.display());
            run(CommandSpec::new("patch")
                .arg("-N")
                .arg("-p1")
                .cwd(&paths.src)
                .stdin_file(&patch))?;
        }
        touch(&paths.stamp.join("patch"))?;
        Ok(())
    }

    fn configure(&self, _ctx: &Context) -> Result<()>;

    fn build(&self, _ctx: &Context) -> Result<()>;

    fn install(&self, _ctx: &Context) -> Result<()>;

    fn clean(&self, ctx: &Context) -> Result<()> {
        let paths = self.paths(ctx);
        println!(
            "[packages][{}] cleaning {}...",
            self.name(),
            paths.root.display()
        );
        if paths.root.exists() {
            fs::remove_dir_all(&paths.root)?;
        }
        Ok(())
    }

    fn run(&self, ctx: &Context, action: Action) -> Result<()> {
        match action {
            Action::Fetch => self.fetch(ctx),
            Action::Patch => self.patch(ctx),
            Action::Configure => self.configure(ctx),
            Action::Build => self.build(ctx),
            Action::Install => self.install(ctx),
            Action::Clean => self.clean(ctx),
            Action::List => Ok(()),
        }
    }
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
        let paths = self.paths(ctx);
        println!("[packages][{}] fetching sources...", self.name());
        ensure_dir(&paths.root)?;
        ensure_dir(&paths.stamp)?;
        if paths.src.is_dir() {
            println!("  reusing existing source tree at {}", paths.src.display());
            touch(&paths.stamp.join("fetch"))?;
            return Ok(());
        }

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
        touch(&paths.stamp.join("fetch"))?;
        Ok(())
    }
}

pub trait GitCloneFetch: Package {
    fn git_url(&self) -> &'static str;
    fn git_commit(&self) -> &'static str;

    fn fetch(&self, ctx: &Context) -> Result<()> {
        let paths = self.paths(ctx);
        println!("[packages][{}] fetching sources...", self.name());
        ensure_dir(&paths.root)?;
        ensure_dir(&paths.stamp)?;
        if paths.src.join(".git").is_dir() {
            println!("  reusing existing clone at {}", paths.src.display());
            touch(&paths.stamp.join("fetch"))?;
            return Ok(());
        }
        if paths.src.exists() {
            fs::remove_dir_all(&paths.src)?;
        }
        println!("  cloning {} at {}...", self.git_url(), self.git_commit());
        run(CommandSpec::new("git")
            .arg("clone")
            .arg(self.git_url())
            .arg(&paths.src))?;
        run(CommandSpec::new("git")
            .arg("checkout")
            .arg(self.git_commit())
            .cwd(&paths.src))?;
        touch(&paths.stamp.join("fetch"))?;
        Ok(())
    }
}
