use crate::r#trait::Package;
use std::fs;

use crate::build::build_relibc;
use crate::command::{CommandSpec, run};
use crate::fs_utils::{ensure_dir, list_patch_files, touch};
use crate::misc::with_stamp;
use crate::types::{Action, Context, PackagePaths, Result};

pub trait MetaPackage: Package {
    fn packages(&self) -> Vec<Box<dyn Package>>;
}

#[macro_export]
macro_rules! make_meta_package {
    ($name: literal, $type: ty, $($package: expr),*) => {
        impl $crate::meta_pkg::MetaPackage for $type {
            fn packages(&self) -> Vec<Box<dyn $crate::r#trait::Package>> {
                vec![$(
                    Box::new($package),
                )*]
            }
        }

        impl $crate::r#trait::Package for $type {
            fn name(&self) -> &'static str {
                $name
            }

            fn fetch(&self, _ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::meta_pkg::meta_panic()
            }

            fn patch(&self, _ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::meta_pkg::meta_panic()
            }

            fn configure(&self, _ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::meta_pkg::meta_panic()
            }

            fn build(&self, _ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::meta_pkg::meta_panic()
            }

            fn install(&self, _ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::meta_pkg::meta_panic()
            }

            fn make(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                for package in self.packages() {
                    package.make(ctx)?;
                }

                Ok(())
            }
        }
    };
}

pub fn meta_panic() -> ! {
    panic!("Meta packages doesnt support this.")
}
