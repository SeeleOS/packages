use crate::{
    fs_utils::{copy_file_with_sudo, verify_same_size},
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
        let paths = self.paths(ctx);
        self.build(ctx)?;
        let source = paths.build.join(self.binary_name());
        let target = ctx.install_dir.join(self.install_name());
        println!(
            "[packages][{}] installing {}...",
            self.name(),
            target.display()
        );
        copy_file_with_sudo(&source, &target)?;
        verify_same_size(&source, &target)?;
        println!("[packages][{}][OK]: installation verified.", self.name());
        Ok(())
    }
}
