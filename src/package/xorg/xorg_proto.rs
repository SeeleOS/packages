use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::XorgUtilMacros;
use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct XorgProto;

impl Package for XorgProto {
    fn name(&self) -> &'static str { "xorg-proto" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(XorgUtilMacros)] }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_meson(self, ctx, &[], Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_meson(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_meson(self, ctx) }
}

impl TarballFetch for XorgProto {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/releases/individual/proto/xorgproto-2025.1.tar.xz"]
    }
}
