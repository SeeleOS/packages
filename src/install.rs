use crate::{
    command::{run, CommandSpec},
    fs_utils::{copy_file_with_sudo, verify_same_size},
    misc::sysroot_dir,
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
        copy_file_with_sudo(&source, &target)?;
        verify_same_size(&source, &target)?;
        Ok(())
    }
}

pub fn install_file(_pkg: &dyn Package, source: &std::path::Path, target: &std::path::Path) -> Result<()> {
    copy_file_with_sudo(source, target)?;
    verify_same_size(source, target)?;
    Ok(())
}

pub fn install_dir_contents(
    _pkg: &dyn Package,
    source_dir: &std::path::Path,
    target_dir: &std::path::Path,
) -> Result<()> {
    run(CommandSpec::new("sudo")
        .arg("mkdir")
        .arg("-p")
        .arg(target_dir))?;
    run(CommandSpec::new("sudo")
        .arg("cp")
        .arg("-a")
        .arg(source_dir.join("."))
        .arg(target_dir))?;
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
