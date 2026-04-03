use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::{LibX11, LibXmu};
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct XorgXrdb;

impl Package for XorgXrdb {
    fn name(&self) -> &'static str { "xorg-xrdb" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(LibX11), Box::new(LibXmu)] }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], &[], Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for XorgXrdb {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/releases/individual/app/xrdb-1.2.2.tar.xz"]
    }
}
