use crate::build::{CC, build_make_in};
use crate::command::CommandSpec;
use crate::configure::{configure_autotools_in, with_envs};
use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::install::install_make_in;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct Ncurses;

const BUILD_CC: &str = "gcc";

impl Package for Ncurses {
    fn name(&self) -> &'static str {
        "ncurses"
    }

    fetch_wrap!(TarballFetch);

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        configure_autotools_in(
            &paths.src,
            with_envs(CommandSpec::new("../configure").cwd(&paths.build), &[("CC", CC)]),
            ctx,
            &[
                "--without-ada",
                "--without-cxx",
                "--without-cxx-binding",
                "--without-manpages",
                "--without-tests",
                "--without-progs",
                "--with-normal",
                "--with-termlib",
                "--disable-db-install",
                "--with-fallbacks=xterm,vt100,linux",
                "--disable-stripping",
                "--disable-widec",
            ],
            vec![
                format!("--with-build-cc={BUILD_CC}"),
            ],
        )
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        build_make_in(&self.calc_paths(ctx).build, &[], Vec::new())
    }

    fn install(&self, ctx: &Context) -> Result<()> { install_make_in(&self.calc_paths(ctx).build, ctx) }
}

impl TarballFetch for Ncurses {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://ftp.gnu.org/gnu/ncurses/ncurses-6.6.tar.gz"]
    }
}
