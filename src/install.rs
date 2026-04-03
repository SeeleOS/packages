use crate::{
    command::{run, CommandSpec},
    fs_utils::{copy_file_with_sudo, verify_same_size},
    misc::sysroot_dir,
    trace::{package, package_detail},
    r#trait::Package,
    types::{Context, Result},
};

#[macro_export]
macro_rules! install_wrap {
    () => {
        fn install(&self, ctx: &Context) -> Result<()> {
            <Self as $crate::install::Install>::install(self, ctx)
        }
    };
}

pub trait Install: Package {
    fn binary_name(&self) -> &'static str;

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let source = paths.build.join(self.binary_name());
        let target = ctx.install_dir.join(self.install_name());
        package(self.name(), format!("installing {}", target.display()));
        package_detail(self.name(), format!("source binary {}", source.display()));
        copy_file_with_sudo(&source, &target)?;
        verify_same_size(&source, &target)?;
        package(self.name(), "installation verified");
        Ok(())
    }
}

pub fn install_file(pkg: &dyn Package, source: &std::path::Path, target: &std::path::Path) -> Result<()> {
    package(pkg.name(), format!("installing {}", target.display()));
    package_detail(pkg.name(), format!("source binary {}", source.display()));
    copy_file_with_sudo(source, target)?;
    verify_same_size(source, target)?;
    package(pkg.name(), "installation verified");
    Ok(())
}

pub fn install_dir_contents(
    pkg: &dyn Package,
    source_dir: &std::path::Path,
    target_dir: &std::path::Path,
) -> Result<()> {
    package(pkg.name(), format!("installing {}", target_dir.display()));
    package_detail(pkg.name(), format!("source dir {}", source_dir.display()));
    run(CommandSpec::new("sudo")
        .arg("mkdir")
        .arg("-p")
        .arg(target_dir))?;
    run(CommandSpec::new("sudo")
        .arg("cp")
        .arg("-a")
        .arg(source_dir.join("."))
        .arg(target_dir))?;
    package(pkg.name(), "directory installation finished");
    Ok(())
}

pub fn install_autotools(pkg: &dyn Package, ctx: &Context) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    install_make_in(&paths.src, ctx)
}

pub fn install_make_in(cwd: &std::path::Path, ctx: &Context) -> Result<()> {
    let sysroot = sysroot_dir(ctx)?;
    run(CommandSpec::new("sudo")
        .arg("env")
        .arg(format!("DESTDIR={}", sysroot.display()))
        .arg("make")
        .arg("-C")
        .arg(cwd)
        .arg("install"))
}

pub fn install_meson(pkg: &dyn Package, ctx: &Context) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    let sysroot = sysroot_dir(ctx)?;
    run(CommandSpec::new("sudo")
        .arg("env")
        .arg(format!("DESTDIR={}", sysroot.display()))
        .arg("meson")
        .arg("install")
        .arg("--no-rebuild")
        .arg("-C")
        .arg(&paths.build))
}
