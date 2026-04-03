use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::{LibXext, LibXt};
use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct LibXmu;

impl Package for LibXmu {
    fn name(&self) -> &'static str { "libxmu" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(LibXext), Box::new(LibXt)] }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for LibXmu {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/lib/libXmu-1.3.1.tar.gz"]
    }
}
