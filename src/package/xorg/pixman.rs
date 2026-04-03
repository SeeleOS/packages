use crate::fetch::GitCloneFetch;
use crate::fetch_wrap;
use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::r#trait::Package;
use crate::types::{Context, Result};

fn pixman_args() -> Vec<String> {
    vec![
        "-Dgtk=disabled".to_string(),
        "-Dlibpng=disabled".to_string(),
        "-Dtests=disabled".to_string(),
    ]
}

pub struct Pixman;

impl Package for Pixman {
    fn name(&self) -> &'static str { "pixman" }
    fetch_wrap!(GitCloneFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_meson(self, ctx, pixman_args(), Vec::new()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_meson(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_meson(self, ctx) }
}

impl GitCloneFetch for Pixman {
    fn git_url(&self) -> &'static str { "https://gitlab.freedesktop.org/pixman/pixman.git" }
    fn git_commit(&self) -> &'static str { "9cc163c9da0fb4da430641715313d95a6ec466d9" }
}
