use crate::build::{CC, build_make_in};
use crate::command::CommandSpec;
use crate::configure::{configure_autotools_in, with_envs};
use crate::install::install_make_in;
use crate::make_package;

const BUILD_CC: &str = "gcc";

make_package!(
    Ncurses,
    "ncurses",
    tarball_url = "https://ftp.gnu.org/gnu/ncurses/ncurses-6.6.tar.gz",
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            configure_autotools_in(
                &paths.src,
                with_envs(
                    CommandSpec::new("../configure").cwd(&paths.build),
                    vec![("CC".to_string(), CC.to_string())],
                ),
                ctx,
                vec![
                    "--without-ada".to_string(),
                    "--without-cxx".to_string(),
                    "--without-cxx-binding".to_string(),
                    "--without-manpages".to_string(),
                    "--without-tests".to_string(),
                    "--without-progs".to_string(),
                    "--with-normal".to_string(),
                    "--with-termlib".to_string(),
                    "--disable-db-install".to_string(),
                    "--with-fallbacks=xterm,xterm-256colors,vt100,linux".to_string(),
                    "--disable-stripping".to_string(),
                    "--disable-widec".to_string(),
                ],
                vec![format!("--with-build-cc={BUILD_CC}")],
            )
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            build_make_in(&self.calc_paths(ctx).build, Vec::new(), Vec::new())
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            install_make_in(&self.calc_paths(ctx).build, ctx)
        }
    }
);
