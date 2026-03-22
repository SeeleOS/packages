use std::fs;

use crate::command::{CommandSpec, run};
use crate::fetch;
use crate::fs_utils::{
    download_file, ensure_dir, list_patch_files, remove_if_exists, remove_path_if_exists, touch,
};
use crate::misc::with_stamp;
use crate::types::{Action, Context, PackagePaths, Result};
pub trait Package {
    fn name(&self) -> &'static str;

    fn install_name(&self) -> &'static str {
        self.name()
    }

    fn calc_paths(&self, ctx: &Context) -> PackagePaths {
        let root = ctx.packages_root.join("work").join(self.name());
        PackagePaths {
            src: root.join("src"),
            stamp: root.join(".stamp"),
            patches: ctx.packages_root.join(self.name()).join("patches"),
            build: root.join("src/build"),
            root,
        }
    }

    fn fetch(&self, _ctx: &Context) -> Result<()>;

    fn patch(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
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
        let paths = self.calc_paths(ctx);
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

    fn make(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);

        with_stamp(|| self.fetch(ctx), "fetch", &paths)?;
        with_stamp(|| self.patch(ctx), "patch", &paths)?;
        with_stamp(|| self.configure(ctx), "configure", &paths)?;
        with_stamp(|| self.build(ctx), "build", &paths)?;
        with_stamp(|| self.install(ctx), "install", &paths)?;

        Ok(())
    }

    fn run(&self, ctx: &Context, action: Action) -> Result<()> {
        match action {
            Action::Install => self.make(ctx),
        }
    }
}
