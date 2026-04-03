use crate::fetch::GitCloneFetch;
use crate::fetch_wrap;
use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::r#trait::Package;
use crate::types::{Context, Result};

const FREETYPE_ARGS: &[&str] = &[
    "-Dbrotli=disabled",
    "-Dbzip2=disabled",
    "-Dharfbuzz=disabled",
    "-Dpng=disabled",
    "-Dzlib=disabled",
];

pub struct Freetype2;

impl Package for Freetype2 {
    fn name(&self) -> &'static str { "freetype2" }
    fetch_wrap!(GitCloneFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_meson(self, ctx, FREETYPE_ARGS, Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_meson(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_meson(self, ctx) }
}

impl GitCloneFetch for Freetype2 {
    fn git_url(&self) -> &'static str { "https://gitlab.freedesktop.org/freetype/freetype.git" }
    fn git_commit(&self) -> &'static str { "0a0221a1347e2f1e07c395263540026e9a0aa7c7" }
}
