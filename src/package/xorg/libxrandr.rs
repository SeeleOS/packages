use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::{LibX11, LibXext, LibXrender, XorgProto};
use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct LibXrandr;

impl Package for LibXrandr {
    fn name(&self) -> &'static str { "libxrandr" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> {
        vec![Box::new(XorgProto), Box::new(LibX11), Box::new(LibXrender), Box::new(LibXext)]
    }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for LibXrandr {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/lib/libXrandr-1.5.5.tar.gz"]
    }
}
