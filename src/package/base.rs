use crate::{
    package::{bash::Bash, busybox::Busybox, tinycc::TinyCc},
    r#trait::Package,
};

pub struct BasePackage;

impl Package for BasePackage {
    fn name(&self) -> &'static str {
        "base"
    }

    fn install_name(&self) -> &'static str {
        "base"
    }

    fn fetch(&self, _ctx: &crate::types::Context) -> crate::types::Result<()> {
        unimplemented!()
    }

    fn configure(&self, _ctx: &crate::types::Context) -> crate::types::Result<()> {
        unimplemented!()
    }

    fn patch(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
        unimplemented!()
    }

    fn build(&self, _ctx: &crate::types::Context) -> crate::types::Result<()> {
        unimplemented!()
    }

    fn install(&self, _ctx: &crate::types::Context) -> crate::types::Result<()> {
        unimplemented!()
    }

    fn make(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
        Bash.make(ctx)?;
        TinyCc.make(ctx)?;
        Busybox.make(ctx)?;

        Ok(())
    }
}
