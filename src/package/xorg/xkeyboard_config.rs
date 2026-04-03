use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::fetch::GitCloneFetch;
use crate::fetch_wrap;
use crate::package::xorg::XorgXkbcomp;
use crate::r#trait::Package;
use crate::types::{Context, Result};

fn xkeyboard_config_args() -> Vec<String> {
    vec!["-Dxorg-rules-symlinks=true".to_string()]
}

pub struct XkeyboardConfig;

impl Package for XkeyboardConfig {
    fn name(&self) -> &'static str { "xkeyboard-config" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> { vec![Box::new(XorgXkbcomp)] }
    fetch_wrap!(GitCloneFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_meson(self, ctx, xkeyboard_config_args(), Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_meson(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_meson(self, ctx) }
}

impl GitCloneFetch for XkeyboardConfig {
    fn git_url(&self) -> &'static str { "https://gitlab.freedesktop.org/xkeyboard-config/xkeyboard-config.git" }
    fn git_commit(&self) -> &'static str { "a79055334104d382bd511760b67acf9a5a161361" }
}
