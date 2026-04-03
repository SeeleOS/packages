use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::fetch::GitCloneFetch;
use crate::fetch_wrap;
use crate::layout::{BINDIR, DEFAULT_FONT_PATH, XKB_DIR, XKB_OUTPUT_DIR};
use crate::package::xorg::hooks::xorg_server_install_hook;
use crate::package::xorg::{LibX11, LibXcb, LibXcvt, LibXdamage, LibXext, LibXfixes, LibXfont2, LibXi, LibXinerama, LibXkbfile, LibXmu, LibXrandr, LibXrender, LibXshmfence, Pixman, XcbProto, XcbUtil, XkeyboardConfig, XorgFontUtil, XorgProto, XorgUtilMacros, XorgXkbcomp, Xtrans};
use crate::r#trait::Package;
use crate::types::{Context, Result};

const XORG_SERVER_FLAGS: &[&str] = &[
    "-Dxorg=true",
    "-Dxv=false",
    "-Dxvfb=false",
    "-Dxephyr=false",
    "-Dxnest=false",
    "-Dsuid_wrapper=false",
    "-Dpciaccess=false",
    "-Ddpms=false",
    "-Dscreensaver=false",
    "-Dxres=false",
    "-Dxvmc=false",
    "-Dsystemd_logind=false",
    "-Dsecure-rpc=false",
    "-Dudev=false",
    "-Dudev_kms=false",
    "-Ddri1=false",
    "-Ddri2=false",
    "-Ddri3=false",
    "-Dint10=false",
    "-Dvgahw=false",
    "-Ddrm=false",
    "-Dglamor=false",
    "-Dglx=false",
    "-Dlisten_tcp=false",
];

fn xorg_server_args() -> Vec<String> {
    vec![
        format!("-Dxkb_bin_dir={BINDIR}"),
        format!("-Dxkb_dir={XKB_DIR}"),
        format!("-Dxkb_output_dir={XKB_OUTPUT_DIR}"),
        format!("-Ddefault_font_path={DEFAULT_FONT_PATH}"),
    ]
}

pub struct XorgServer;

impl Package for XorgServer {
    fn name(&self) -> &'static str { "xorg-server" }
    fn dependencies(&self) -> Vec<Box<dyn Package>> {
        vec![Box::new(XorgUtilMacros), Box::new(XorgProto), Box::new(XorgFontUtil), Box::new(XcbProto), Box::new(XcbUtil), Box::new(Xtrans), Box::new(LibXinerama), Box::new(LibXcvt), Box::new(LibXshmfence), Box::new(LibX11), Box::new(LibXkbfile), Box::new(LibXmu), Box::new(LibXfont2), Box::new(LibXi), Box::new(LibXrender), Box::new(LibXrandr), Box::new(LibXcb), Box::new(LibXfixes), Box::new(LibXext), Box::new(LibXdamage), Box::new(Pixman), Box::new(XorgXkbcomp), Box::new(XkeyboardConfig)]
    }
    fetch_wrap!(GitCloneFetch);
    fn configure(&self, ctx: &Context) -> Result<()> { configure_meson(self, ctx, XORG_SERVER_FLAGS, xorg_server_args()) }
    fn build(&self, ctx: &Context) -> Result<()> { build_meson(self, ctx) }
    fn install(&self, ctx: &Context) -> Result<()> { install_meson(self, ctx)?; xorg_server_install_hook(ctx) }
}

impl GitCloneFetch for XorgServer {
    fn git_url(&self) -> &'static str { "https://gitlab.freedesktop.org/xorg/xserver.git" }
    fn git_commit(&self) -> &'static str { "312a25c65c8a918fea2cc77abd0db07ec0fc421c" }
}
