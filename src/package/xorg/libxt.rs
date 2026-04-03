use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::{LibSm, LibX11};
use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::layout::APPDEFAULTDIR;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct LibXt;

fn libxt_args() -> Vec<String> {
    vec![format!("--with-appdefaultdir={APPDEFAULTDIR}")]
}

impl Package for LibXt {
    fn name(&self) -> &'static str { "libxt" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(LibX11), Box::new(LibSm)] }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_autotools(self, ctx, &[], libxt_args()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for LibXt {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/lib/libXt-1.3.1.tar.gz"]
    }
}
