use crate::r#trait::Package;

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
                $crate::trace::package(self.name(), "starting meta-package install workflow");
                for package in <Self as $crate::meta_pkg::MetaPackage>::packages(self) {
                    $crate::trace::package_detail(
                        self.name(),
                        format!("delegating to child package `{}`", package.name()),
                    );
                    package.make(ctx)?;
                }
                $crate::trace::package(self.name(), "meta-package install workflow complete");

                Ok(())
            }
        }
    };
}

pub fn meta_panic() -> ! {
    panic!("Meta packages doesnt support this.")
}
