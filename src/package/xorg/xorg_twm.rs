use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::LibXmu;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct XorgTwm;

impl Package for XorgTwm {
    fn name(&self) -> &'static str { "xorg-twm" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(LibXmu)] }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for XorgTwm {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/pub/individual/app/twm-1.0.13.1.tar.xz"]
    }
}
