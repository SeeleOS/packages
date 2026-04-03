use crate::fetch::GitCloneFetch;
use crate::fetch_wrap;
use crate::package::xorg::{LibX11, XorgProto};
use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct LibXfixes;

impl Package for LibXfixes {
    fn name(&self) -> &'static str { "libxfixes" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(XorgProto), Box::new(LibX11)] }
    fetch_wrap!(GitCloneFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_meson(self, ctx, Vec::new(), Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_meson(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_meson(self, ctx) }
}

impl GitCloneFetch for LibXfixes {
    fn git_url(&self) -> &'static str { "https://gitlab.freedesktop.org/xorg/lib/libxfixes.git" }
    fn git_commit(&self) -> &'static str { "70d5b0e37f8a759f3dbc218f22954347ceed094a" }
}
