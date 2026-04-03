use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::LibXcb;
use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct XcbUtil;

impl Package for XcbUtil {
    fn name(&self) -> &'static str { "xcb-util" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(LibXcb)] }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], &[], Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for XcbUtil {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://xcb.freedesktop.org/dist/xcb-util-0.4.1.tar.xz"]
    }
}
