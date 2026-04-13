use std::process::Output;

use std::fs;

use crate::build::build_relibc;
use crate::command::{CommandSpec, run_output};
use crate::fs_utils::{ensure_dir, list_patch_files};
use crate::install::deploy_sysroot;
use crate::misc::with_stamp;
use crate::types::{Action, Context, PackagePaths, Result};

fn patch_output_text(output: &Output) -> String {
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

fn patch_command<'a>(src_dir: &'a std::path::Path, patch: &'a std::path::Path) -> CommandSpec<'a> {
    CommandSpec::new("patch")
        .arg("-N")
        .arg("--forward")
        .arg("--batch")
        .arg("-p1")
        .cwd(src_dir)
        .stdin_file(patch)
}

pub fn apply_patch_file(src_dir: &std::path::Path, patch: &std::path::Path) -> Result<()> {
    let dry_run = run_output(patch_command(src_dir, patch).arg("--dry-run"))?;
    if dry_run.status.success() {
        let apply = run_output(patch_command(src_dir, patch))?;
        if !apply.status.success() {
            let output = patch_output_text(&apply);
            return Err(format!("patch {} failed:\n{}", patch.display(), output).into());
        }
        return Ok(());
    }

    let dry_run_text = patch_output_text(&dry_run);
    if dry_run_text.contains("Reversed (or previously applied) patch detected") {
        eprintln!(
            "[packages] skipping already applied patch {}",
            patch.display()
        );
        return Ok(());
    }

    Err(format!("patch {} failed:\n{}", patch.display(), dry_run_text).into())
}

pub fn apply_pkg_specific_patches(paths: &PackagePaths) -> Result<()> {
    let config_patch = paths.pkg_specific.join("config.patch");
    if config_patch.is_file() {
        apply_patch_file(&paths.src, &config_patch)?;
    }

    let mut patch_files = list_patch_files(&paths.pkg_specific)?;
    patch_files.sort();
    for patch in patch_files {
        if patch == config_patch {
            continue;
        }
        apply_patch_file(&paths.src, &patch)?;
    }

    Ok(())
}

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
            patch: ctx
                .packages_root
                .join("patches")
                .join(format!("{}.patch", self.name())),
            build: root.join("src/build"),
            pkg_specific: ctx.pkg_specific_root.join(self.name()),
            root,
        }
    }

    fn fetch(&self, _ctx: &Context) -> Result<()>;

    fn patch(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        ensure_dir(&paths.stamp)?;
        if paths.patch.exists() {
            apply_patch_file(&paths.src, &paths.patch)?;
        }
        Ok(())
    }

    fn configure(&self, _ctx: &Context) -> Result<()>;

    fn build(&self, _ctx: &Context) -> Result<()>;

    fn install(&self, _ctx: &Context) -> Result<()>;

    fn clean(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        if paths.root.exists() {
            fs::remove_dir_all(&paths.root)?;
        }
        Ok(())
    }

    fn dependencies(&self) -> Vec<Box<dyn Package>> {
        Vec::new()
    }

    fn make(&self, ctx: &Context) -> Result<()> {
        if !ctx.ignore_deps {
            for dep in self.dependencies() {
                dep.make(ctx)?;
            }
        }

        build_relibc(ctx)?;

        let paths = self.calc_paths(ctx);
        paths.ensure()?;
        with_stamp(|| self.fetch(ctx), "fetch", &paths, ctx.rebuild, true)?;
        paths.ensure()?;
        with_stamp(|| self.patch(ctx), "patch", &paths, ctx.rebuild, false)?;
        paths.ensure()?;
        with_stamp(
            || self.configure(ctx),
            "configure",
            &paths,
            ctx.rebuild,
            false,
        )?;
        paths.ensure()?;
        with_stamp(|| self.build(ctx), "build", &paths, ctx.rebuild, false)?;
        paths.ensure()?;
        with_stamp(|| self.install(ctx), "install", &paths, ctx.rebuild, false)?;

        Ok(())
    }

    fn run(&self, ctx: &Context, action: Action) -> Result<()> {
        match action {
            Action::Install => {
                self.make(ctx)?;
                deploy_sysroot(ctx)
            }
            Action::Clean => self.clean(ctx),
            Action::RebuildOnly => {
                self.clean(ctx)?;
                self.make(ctx)?;
                deploy_sysroot(ctx)
            }
        }
    }
}
