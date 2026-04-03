use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::XorgProto;
use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct LibXshmfence;

impl Package for LibXshmfence {
    fn name(&self) -> &'static str { "libxshmfence" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(XorgProto)] }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for LibXshmfence {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/lib/libxshmfence-1.3.3.tar.gz"]
    }
}
