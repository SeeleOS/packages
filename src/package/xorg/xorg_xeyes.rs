use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::fetch::GitCloneFetch;
use crate::fetch_wrap;
use crate::package::xorg::{LibX11, LibXcb, LibXext, LibXi, LibXmu, LibXrender, LibXt};
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct XorgXeyes;

impl Package for XorgXeyes {
    fn name(&self) -> &'static str { "xorg-xeyes" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> {
        vec![Box::new(LibX11), Box::new(LibXt), Box::new(LibXext), Box::new(LibXmu), Box::new(LibXrender), Box::new(LibXi), Box::new(LibXcb)]
    }
    fetch_wrap!(GitCloneFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_meson(self, ctx, Vec::new(), Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_meson(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_meson(self, ctx) }
}

impl GitCloneFetch for XorgXeyes {
    fn git_url(&self) -> &'static str { "https://gitlab.freedesktop.org/xorg/app/xeyes.git" }
    fn git_commit(&self) -> &'static str { "7dc4f720f57471d2eccefd87cdca54494cf75eb5" }
}
