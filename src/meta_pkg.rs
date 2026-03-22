use crate::r#trait::Package;
use std::fs;

use crate::build::build_relibc;
use crate::command::{CommandSpec, run};
use crate::fs_utils::{ensure_dir, list_patch_files, touch};
use crate::misc::with_stamp;
use crate::types::{Action, Context, PackagePaths, Result};

pub trait MetaPackage: Package {
    fn packages(&self) -> Vec<impl Package>;
    fn fetch(&self, _ctx: &Context) -> Result<()> {
        meta_panic()
    }

    fn patch(&self, ctx: &Context) -> Result<()> {
        meta_panic()
    }

    fn configure(&self, _ctx: &Context) -> Result<()> {
        meta_panic()
    }

    fn build(&self, _ctx: &Context) -> Result<()> {
        meta_panic()
    }

    fn install(&self, _ctx: &Context) -> Result<()> {
        meta_panic()
    }

    fn make(&self, ctx: &Context) -> Result<()> {
        for package in self.packages() {
            package.make(ctx)?;
        }

        Ok(())
    }
}

fn meta_panic() -> ! {
    panic!("Meta packages doesnt support this.")
}
