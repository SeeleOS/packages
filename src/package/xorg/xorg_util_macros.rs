use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct XorgUtilMacros;

impl Package for XorgUtilMacros {
    fn name(&self) -> &'static str { "xorg-util-macros" }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for XorgUtilMacros {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/util/util-macros-1.20.2.tar.gz"]
    }
}
