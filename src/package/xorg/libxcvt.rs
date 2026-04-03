use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct LibXcvt;

impl Package for LibXcvt {
    fn name(&self) -> &'static str { "libxcvt" }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_meson(self, ctx, Vec::new(), Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_meson(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_meson(self, ctx) }
}

impl TarballFetch for LibXcvt {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/lib/libxcvt-0.1.3.tar.xz"]
    }
}
