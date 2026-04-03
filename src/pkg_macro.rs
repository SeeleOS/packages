#[macro_export]
macro_rules! make_autotools_package {
    (
        $ty:ident,
        $name:literal,
        tarball_url = $tarball_url:expr
        $(, dependencies = [$($dep:path),* $(,)?])?
        $(,)?
    ) => {
        pub struct $ty;

        impl $crate::r#trait::Package for $ty {
            fn name(&self) -> &'static str { $name }

            fn dependencies(&self) -> Vec<Box<dyn $crate::r#trait::Package>> {
                vec![
                    $(
                        $(Box::new($dep)),*
                    )?
                ]
            }

            fn fetch(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                <$ty as $crate::fetch::TarballFetch>::fetch(self, ctx)
            }

            fn configure(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::configure::configure_autotools(self, ctx, &[], &[], Vec::new())
            }

            fn build(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::build::build_autotools(self, ctx)
            }

            fn install(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::install::install_autotools(self, ctx)
            }
        }

        impl $crate::fetch::TarballFetch for $ty {
            fn tarball_url(&self) -> Vec<&'static str> {
                vec![$tarball_url]
            }
        }
    };
    (
        $ty:ident,
        $name:literal,
        git_url = $git_url:expr,
        git_commit = $git_commit:expr
        $(, dependencies = [$($dep:path),* $(,)?])?
        $(,)?
    ) => {
        pub struct $ty;

        impl $crate::r#trait::Package for $ty {
            fn name(&self) -> &'static str { $name }

            fn dependencies(&self) -> Vec<Box<dyn $crate::r#trait::Package>> {
                vec![
                    $(
                        $(Box::new($dep)),*
                    )?
                ]
            }

            fn fetch(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                <$ty as $crate::fetch::GitCloneFetch>::fetch(self, ctx)
            }

            fn configure(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::configure::configure_autotools(self, ctx, &[], &[], Vec::new())
            }

            fn build(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::build::build_autotools(self, ctx)
            }

            fn install(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::install::install_autotools(self, ctx)
            }
        }

        impl $crate::fetch::GitCloneFetch for $ty {
            fn git_url(&self) -> &'static str { $git_url }

            fn git_commit(&self) -> &'static str { $git_commit }
        }
    };
}
