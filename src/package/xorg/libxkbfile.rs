use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::{LibX11, XorgProto, XorgUtilMacros};
use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct LibXkbfile;

impl Package for LibXkbfile {
    fn name(&self) -> &'static str { "libxkbfile" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> {
        vec![Box::new(XorgUtilMacros), Box::new(XorgProto), Box::new(LibX11)]
    }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_meson(self, ctx, Vec::new(), Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_meson(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_meson(self, ctx) }
}

impl TarballFetch for LibXkbfile {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/lib/libxkbfile-1.2.0.tar.xz"]
    }
}
