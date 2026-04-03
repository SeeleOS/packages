use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::XorgUtilMacros;
use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct Xtrans;

impl Package for Xtrans {
    fn name(&self) -> &'static str { "xtrans" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(XorgUtilMacros)] }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for Xtrans {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/lib/xtrans-1.6.0.tar.gz"]
    }
}
