#[macro_export]
macro_rules! package_phase {
    ($body:block) => {{
        $body
        Ok(())
    }};
}

#[macro_export]
macro_rules! make_package {
    (
        $ty:ident,
        $name:literal,
        tarball_url = $tarball_url:expr
        $(, dependencies = [$($dep:path),* $(,)?])?
        ,
        package_impl = { $($impls:tt)* }
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

            $($impls)*
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
        ,
        package_impl = { $($impls:tt)* }
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

            $($impls)*
        }

        impl $crate::fetch::GitCloneFetch for $ty {
            fn git_url(&self) -> &'static str { $git_url }

            fn git_commit(&self) -> &'static str { $git_commit }
        }
    };
    (
        $ty:ident,
        $name:literal,
        git_url = $git_url:expr
        $(, dependencies = [$($dep:path),* $(,)?])?
        ,
        package_impl = { $($impls:tt)* }
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
                <$ty as $crate::fetch::GitHeadFetch>::fetch(self, ctx)
            }

            $($impls)*
        }

        impl $crate::fetch::GitHeadFetch for $ty {
            fn git_url(&self) -> &'static str { $git_url }
        }
    };
}

#[macro_export]
macro_rules! make_autotools_package {
    (
        $ty:ident,
        $name:literal,
        $fetch_key:ident = $fetch_value:expr
        $(, git_commit = $git_commit:expr)?
        $(, dependencies = [$($dep:path),* $(,)?])?
        $(, configure = { $($cfg:tt)* })?
        $(, build = { $($build:tt)* })?
        $(, configure_override = $configure_override:block)?
        $(, build_override = $build_override:block)?
        $(, install_override = $install_override:block)?
        $(,)?
    ) => {
        $crate::make_package!(
            $ty,
            $name,
            $fetch_key = $fetch_value
            $(, git_commit = $git_commit)?
            $(, dependencies = [$($dep),*])?,
            package_impl = {
                fn configure(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($configure_override);
                    )?
                    $crate::configure::configure_autotools(
                        self,
                        ctx,
                        $crate::make_autotools_package!(@cfg_env_from [ $($($cfg)*)? ]),
                        $crate::make_autotools_package!(@cfg_args_from [ $($($cfg)*)? ]),
                        ($crate::make_autotools_package!(@cfg_dynamic_args_from [ $($($cfg)*)? ]))(ctx),
                    )
                }

                fn build(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($build_override);
                    )?
                    $crate::build::build_autotools_with(
                        self,
                        ctx,
                        $crate::make_autotools_package!(@build_env_from [ $($($build)*)? ]),
                        $crate::make_autotools_package!(@build_args_from [ $($($build)*)? ]),
                    )
                }

                fn install(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($install_override);
                    )?
                    $crate::install::install_autotools(self, ctx)
                }
            }
        );
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

#[macro_export]
macro_rules! make_meson_package {
    (
        $ty:ident,
        $name:literal,
        $fetch_key:ident = $fetch_value:expr
        $(, git_commit = $git_commit:expr)?
        $(, dependencies = [$($dep:path),* $(,)?])?
        $(, configure = { $($cfg:tt)* })?
        $(, configure_override = $configure_override:block)?
        $(, build_override = $build_override:block)?
        $(, install_override = $install_override:block)?
        $(,)?
    ) => {
        $crate::make_package!(
            $ty,
            $name,
            $fetch_key = $fetch_value
            $(, git_commit = $git_commit)?
            $(, dependencies = [$($dep),*])?,
            package_impl = {
                fn configure(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($configure_override);
                    )?
                    $crate::configure::configure_meson(
                        self,
                        ctx,
                        $crate::make_meson_package!(@cfg_env_from [ $($($cfg)*)? ]),
                        $crate::make_meson_package!(@cfg_args_from [ $($($cfg)*)? ]),
                        ($crate::make_meson_package!(@cfg_dynamic_args_from [ $($($cfg)*)? ]))(ctx),
                    )
                }

                fn build(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($build_override);
                    )?
                    $crate::build::build_meson(self, ctx)
                }

                fn install(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($install_override);
                    )?
                    $crate::install::install_meson(self, ctx)
                }
            }
        );
    };
    (@cfg_env_from [ $($items:tt)* ]) => { $crate::make_meson_package!(@find_cfg_env [ $($items)* ]) };
    (@find_cfg_env [ ]) => { Vec::<(String, String)>::new() };
    (@find_cfg_env [ env = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_cfg_env [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_meson_package!(@find_cfg_env [ $($rest)* ])
    };
    (@find_cfg_env [ $key:ident = $value:expr ]) => { Vec::<(String, String)>::new() };
    (@cfg_args_from [ $($items:tt)* ]) => { $crate::make_meson_package!(@find_cfg_args [ $($items)* ]) };
    (@find_cfg_args [ ]) => { Vec::<String>::new() };
    (@find_cfg_args [ args = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_cfg_args [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_meson_package!(@find_cfg_args [ $($rest)* ])
    };
    (@find_cfg_args [ $key:ident = $value:expr ]) => { Vec::<String>::new() };
    (@cfg_dynamic_args_from [ $($items:tt)* ]) => { $crate::make_meson_package!(@find_cfg_dynamic_args [ $($items)* ]) };
    (@find_cfg_dynamic_args [ ]) => { |_| Vec::new() };
    (@find_cfg_dynamic_args [ dynamic_args = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_cfg_dynamic_args [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_meson_package!(@find_cfg_dynamic_args [ $($rest)* ])
    };
    (@find_cfg_dynamic_args [ $key:ident = $value:expr ]) => { |_| Vec::new() };
}

#[macro_export]
macro_rules! make_meson_packages {
    ($({ $($inner:tt)* }),* $(,)?) => {
        $(
            $crate::make_meson_package!($($inner)*);
        )*
    };
}

#[macro_export]
macro_rules! make_cmake_package {
    (
        $ty:ident,
        $name:literal,
        $fetch_key:ident = $fetch_value:expr
        $(, git_commit = $git_commit:expr)?
        $(, dependencies = [$($dep:path),* $(,)?])?
        $(, configure = { $($cfg:tt)* })?
        $(, configure_override = $configure_override:block)?
        $(, build_override = $build_override:block)?
        $(, install_override = $install_override:block)?
        $(,)?
    ) => {
        $crate::make_package!(
            $ty,
            $name,
            $fetch_key = $fetch_value
            $(, git_commit = $git_commit)?
            $(, dependencies = [$($dep),*])?,
            package_impl = {
                fn configure(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($configure_override);
                    )?
                    $crate::configure::configure_cmake(
                        self,
                        ctx,
                        $crate::make_cmake_package!(@cfg_args_from [ $($($cfg)*)? ]),
                        ($crate::make_cmake_package!(@cfg_dynamic_args_from [ $($($cfg)*)? ]))(ctx),
                    )
                }

                fn build(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($build_override);
                    )?
                    $crate::build::build_cmake(self, ctx)
                }

                fn install(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($install_override);
                    )?
                    $crate::install::install_cmake(self, ctx)
                }
            }
        );
    };
    (@cfg_args_from [ $($items:tt)* ]) => { $crate::make_cmake_package!(@find_cfg_args [ $($items)* ]) };
    (@find_cfg_args [ ]) => { Vec::<String>::new() };
    (@find_cfg_args [ args = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_cfg_args [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_cmake_package!(@find_cfg_args [ $($rest)* ])
    };
    (@find_cfg_args [ $key:ident = $value:expr ]) => { Vec::<String>::new() };
    (@cfg_dynamic_args_from [ $($items:tt)* ]) => { $crate::make_cmake_package!(@find_cfg_dynamic_args [ $($items)* ]) };
    (@find_cfg_dynamic_args [ ]) => { |_| Vec::new() };
    (@find_cfg_dynamic_args [ dynamic_args = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_cfg_dynamic_args [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_cmake_package!(@find_cfg_dynamic_args [ $($rest)* ])
    };
    (@find_cfg_dynamic_args [ $key:ident = $value:expr ]) => { |_| Vec::new() };
}

#[macro_export]
macro_rules! make_cmake_packages {
    ($({ $($inner:tt)* }),* $(,)?) => {
        $(
            $crate::make_cmake_package!($($inner)*);
        )*
    };
}

#[macro_export]
macro_rules! make_cargo_package {
    (
        $ty:ident,
        $name:literal,
        $fetch_key:ident = $fetch_value:expr
        $(, git_commit = $git_commit:expr)?
        $(, dependencies = [$($dep:path),* $(,)?])?
        $(, build = { $($build:tt)* })?
        $(, install = { $($install:tt)* })?
        $(, configure_override = $configure_override:block)?
        $(, build_override = $build_override:block)?
        $(, install_override = $install_override:block)?
        $(,)?
    ) => {
        $crate::make_package!(
            $ty,
            $name,
            $fetch_key = $fetch_value
            $(, git_commit = $git_commit)?
            $(, dependencies = [$($dep),*])?,
            package_impl = {
                fn configure(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($configure_override);
                    )?
                    Ok(())
                }

                fn build(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($build_override);
                    )?
                    $crate::build::build_cargo_with(
                        self,
                        ctx,
                        $crate::make_cargo_package!(@build_env_from [ $($($build)*)? ]),
                        $crate::make_cargo_package!(@build_args_from [ $($($build)*)? ]),
                    )
                }

                fn install(&self, ctx: &$crate::types::Context) -> $crate::types::Result<()> {
                    $(
                        return $crate::package_phase!($install_override);
                    )?
                    $crate::install::install_cargo(
                        self,
                        ctx,
                        $crate::make_cargo_package!(@install_bins_from [ $($($install)*)? ]),
                        $crate::make_cargo_package!(@install_profile_from [ $($($install)*)? ]),
                    )
                }
            }
        );
    };
    (@build_env_from [ $($items:tt)* ]) => { $crate::make_cargo_package!(@find_build_env [ $($items)* ]) };
    (@find_build_env [ ]) => { Vec::<(String, String)>::new() };
    (@find_build_env [ env = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_build_env [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_cargo_package!(@find_build_env [ $($rest)* ])
    };
    (@find_build_env [ $key:ident = $value:expr ]) => { Vec::<(String, String)>::new() };
    (@build_args_from [ $($items:tt)* ]) => { $crate::make_cargo_package!(@find_build_args [ $($items)* ]) };
    (@find_build_args [ ]) => { Vec::<String>::new() };
    (@find_build_args [ args = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_build_args [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_cargo_package!(@find_build_args [ $($rest)* ])
    };
    (@find_build_args [ $key:ident = $value:expr ]) => { Vec::<String>::new() };
    (@install_bins_from [ $($items:tt)* ]) => { $crate::make_cargo_package!(@find_install_bins [ $($items)* ]) };
    (@find_install_bins [ ]) => { Vec::<String>::new() };
    (@find_install_bins [ bins = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_install_bins [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_cargo_package!(@find_install_bins [ $($rest)* ])
    };
    (@find_install_bins [ $key:ident = $value:expr ]) => { Vec::<String>::new() };
    (@install_profile_from [ $($items:tt)* ]) => { $crate::make_cargo_package!(@find_install_profile [ $($items)* ]) };
    (@find_install_profile [ ]) => { "debug" };
    (@find_install_profile [ profile = $value:expr $(, $($rest:tt)*)? ]) => { $value };
    (@find_install_profile [ $key:ident = $value:expr, $($rest:tt)* ]) => {
        $crate::make_cargo_package!(@find_install_profile [ $($rest)* ])
    };
    (@find_install_profile [ $key:ident = $value:expr ]) => { "debug" };
}
