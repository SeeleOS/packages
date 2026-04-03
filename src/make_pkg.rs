#[macro_export]
macro_rules! make_autotools_package {
    (
        $ty:ident,
        $name:literal,
        tarball_url = $tarball_url:expr
        $(, dependencies = [$($dep:path),* $(,)?])?
        $(, configure = { $($cfg:tt)* })?
        $(, build = { $($build:tt)* })?
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
                $crate::configure::configure_autotools(
                    self,
                    ctx,
                    $crate::make_autotools_package!(@cfg_env_from [ $($($cfg)*)? ]),
                    $crate::make_autotools_package!(@cfg_args_from [ $($($cfg)*)? ]),
                    ($crate::make_autotools_package!(@cfg_dynamic_args_from [ $($($cfg)*)? ]))(ctx),
                )
            }

            fn build(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::build::build_autotools_with(
                    self,
                    ctx,
                    $crate::make_autotools_package!(@build_env_from [ $($($build)*)? ]),
                    $crate::make_autotools_package!(@build_args_from [ $($($build)*)? ]),
                )
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
        $(, configure = { $($cfg:tt)* })?
        $(, build = { $($build:tt)* })?
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
                $crate::configure::configure_autotools(
                    self,
                    ctx,
                    $crate::make_autotools_package!(@cfg_env_from [ $($($cfg)*)? ]),
                    $crate::make_autotools_package!(@cfg_args_from [ $($($cfg)*)? ]),
                    ($crate::make_autotools_package!(@cfg_dynamic_args_from [ $($($cfg)*)? ]))(ctx),
                )
            }

            fn build(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                $crate::build::build_autotools_with(
                    self,
                    ctx,
                    $crate::make_autotools_package!(@build_env_from [ $($($build)*)? ]),
                    $crate::make_autotools_package!(@build_args_from [ $($($build)*)? ]),
                )
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
    (@cfg_env_from [ $($items:tt)* ]) => { $crate::make_autotools_package!(@find_cfg_env [ $($items)* ]) };
    (@find_cfg_env [ ]) => { Vec::<(String, String)>::new() };
    (@find_cfg_env [ env = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_cfg_env [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_autotools_package!(@find_cfg_env [ $($rest)* ])
    };
    (@find_cfg_env [ $key:ident = $value:expr ]) => { Vec::<(String, String)>::new() };

    (@cfg_args_from [ $($items:tt)* ]) => { $crate::make_autotools_package!(@find_cfg_args [ $($items)* ]) };
    (@find_cfg_args [ ]) => { Vec::<String>::new() };
    (@find_cfg_args [ args = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_cfg_args [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_autotools_package!(@find_cfg_args [ $($rest)* ])
    };
    (@find_cfg_args [ $key:ident = $value:expr ]) => { Vec::<String>::new() };

    (@cfg_dynamic_args_from [ $($items:tt)* ]) => { $crate::make_autotools_package!(@find_cfg_dynamic_args [ $($items)* ]) };
    (@find_cfg_dynamic_args [ ]) => { |_| Vec::new() };
    (@find_cfg_dynamic_args [ dynamic_args = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_cfg_dynamic_args [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_autotools_package!(@find_cfg_dynamic_args [ $($rest)* ])
    };
    (@find_cfg_dynamic_args [ $key:ident = $value:expr ]) => { |_| Vec::new() };

    (@build_env_from [ $($items:tt)* ]) => { $crate::make_autotools_package!(@find_build_env [ $($items)* ]) };
    (@find_build_env [ ]) => { Vec::<(String, String)>::new() };
    (@find_build_env [ env = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_build_env [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_autotools_package!(@find_build_env [ $($rest)* ])
    };
    (@find_build_env [ $key:ident = $value:expr ]) => { Vec::<(String, String)>::new() };

    (@build_args_from [ $($items:tt)* ]) => { $crate::make_autotools_package!(@find_build_args [ $($items)* ]) };
    (@find_build_args [ ]) => { Vec::<String>::new() };
    (@find_build_args [ args = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_build_args [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_autotools_package!(@find_build_args [ $($rest)* ])
    };
    (@find_build_args [ $key:ident = $value:expr ]) => { Vec::<String>::new() };
}

#[macro_export]
macro_rules! make_autotools_packages {
    ($({ $($inner:tt)* }),* $(,)?) => {
        $(
            $crate::make_autotools_package!($($inner)*);
        )*
    };
}
