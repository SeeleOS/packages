use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::package::xorg::{LibXcb, XorgProto, Xtrans};
use crate::build::build_autotools;
use crate::configure::configure_autotools;
use crate::install::install_autotools;
use crate::r#trait::Package;
use crate::types::{Context, Result};

fn libx11_extra_args(ctx: &Context) -> Vec<String> {
    vec![format!(
        "--with-keysymdefdir={}",
        ctx.system_include_dir.join("X11").display()
    )]
}

pub struct LibX11;

impl Package for LibX11 {
    fn name(&self) -> &'static str { "libx11" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> {
        vec![Box::new(XorgProto), Box::new(LibXcb), Box::new(Xtrans)]
    }
    fetch_wrap!(TarballFetch);
    fn configure(&self, ctx: &Context) -> Result<()> {
        configure_autotools(self, ctx, &[], libx11_extra_args(ctx))
    }
    fn build(&self, ctx: &Context) -> Result<()> { build_autotools(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_autotools(self, ctx) }
}

impl TarballFetch for LibX11 {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://www.x.org/archive/individual/lib/libX11-1.8.13.tar.xz"]
    }
}
