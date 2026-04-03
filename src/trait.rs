use std::fs;

use crate::build::build_relibc;
use crate::command::{CommandSpec, run};
use crate::fs_utils::{ensure_dir, list_patch_files};
use crate::misc::{mount_sysroot, with_stamp};
use crate::trace::{package, package_detail, section};
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
        let mut patches = list_patch_files(&paths.patches)?;
        if patches.is_empty() {
            package(self.name(), "no patches to apply");
            return Ok(());
        }
        package(
            self.name(),
            format!("applying patches from {}", paths.patches.display()),
        );
        patches.sort();
        for patch in patches {
            package_detail(self.name(), format!("applying patch {}", patch.display()));
            run(CommandSpec::new("patch")
                .arg("-N")
                .arg("-p1")
                .cwd(&paths.src)
                .stdin_file(&patch))?;
        }
        package(self.name(), "patch phase complete");
        Ok(())
    }

    fn configure(&self, _ctx: &Context) -> Result<()>;

    fn build(&self, _ctx: &Context) -> Result<()>;

    fn install(&self, _ctx: &Context) -> Result<()>;

    fn clean(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        package(self.name(), format!("cleaning {}", paths.root.display()));
        if paths.root.exists() {
            fs::remove_dir_all(&paths.root)?;
            package_detail(self.name(), "workspace removed");
        } else {
            package_detail(self.name(), "workspace already absent");
        }
        Ok(())
    }

    fn dependencies(&self) -> Vec<Box<dyn Package>> {
        Vec::new()
    }

    fn make(&self, ctx: &Context) -> Result<()> {
        section(format!(
            "begin install workflow for package `{}`",
            self.name()
        ));

        if !ctx.ignore_deps {
            for dep in self.dependencies() {
                dep.make(ctx)?;
            }
        }

        build_relibc(ctx)?;

        let paths = self.calc_paths(ctx);
        package_detail(
            self.name(),
            format!(
                "workspace root={} src={} build={} stamp={}",
                paths.root.display(),
                paths.src.display(),
                paths.build.display(),
                paths.stamp.display()
            ),
        );

        mount_sysroot()?;

        package(self.name(), "phase: ensure workspace");
        paths.ensure()?;
        package(self.name(), "phase: fetch");
        with_stamp(|| self.fetch(ctx), "fetch", &paths, ctx.rebuild, true)?;
        package(self.name(), "phase: ensure workspace");
        paths.ensure()?;
        package(self.name(), "phase: patch");
        with_stamp(|| self.patch(ctx), "patch", &paths, ctx.rebuild, false)?;
        package(self.name(), "phase: ensure workspace");
        paths.ensure()?;
        package(self.name(), "phase: configure");
        with_stamp(|| self.configure(ctx), "configure", &paths, ctx.rebuild, false)?;
        package(self.name(), "phase: ensure workspace");
        paths.ensure()?;
        package(self.name(), "phase: build");
        with_stamp(|| self.build(ctx), "build", &paths, ctx.rebuild, false)?;
        package(self.name(), "phase: ensure workspace");
        paths.ensure()?;
        package(self.name(), "phase: install");
        with_stamp(|| self.install(ctx), "install", &paths, ctx.rebuild, false)?;
        package(self.name(), "install workflow complete");

        Ok(())
    }

    fn run(&self, ctx: &Context, action: Action) -> Result<()> {
        package(self.name(), format!("dispatching action {:?}", action));
        match action {
            Action::Install => self.make(ctx),
            Action::Clean => self.clean(ctx),
        }
    }
}
