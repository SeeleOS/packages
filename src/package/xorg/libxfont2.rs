use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::{Freetype2, LibFontenc, LibX11, XorgProto, XorgUtilMacros, Xtrans};
use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::r#trait::Package;
use crate::types::{Context, Result};

const LIBXFONT2_ARGS: &[&str] = &["--disable-devel-docs", "--disable-selective-werror"];

pub struct LibXfont2;

impl Package for LibXfont2 {
    fn name(&self) -> &'static str { "libxfont2" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> {
        vec![Box::new(XorgUtilMacros), Box::new(XorgProto), Box::new(LibX11), Box::new(Xtrans), Box::new(Freetype2), Box::new(LibFontenc)]
    }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], LIBXFONT2_ARGS, Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for LibXfont2 {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/lib/libXfont2-2.0.7.tar.gz"]
    }
}
