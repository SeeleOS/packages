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

pub fn install_autotools(pkg: &dyn Package, ctx: &Context) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    let sysroot = sysroot_dir(ctx)?;
    run(CommandSpec::new("sudo")
        .arg("env")
        .arg(format!("DESTDIR={}", sysroot.display()))
        .arg("make")
        .arg("-C")
        .arg(&paths.src)
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
