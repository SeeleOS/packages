use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::install::install_meson;
use crate::layout::{BINDIR, DEFAULT_FONT_PATH, XKB_DIR, XKB_OUTPUT_DIR};
use crate::make_package;
use crate::package::xorg::hooks::xorg_server_install_hook;
use crate::package::xorg::{
    LibX11, LibXcb, LibXcvt, LibXdamage, LibXext, LibXfixes, LibXfont2, LibXi, LibXinerama,
    LibXkbfile, LibXmu, LibXrandr, LibXrender, LibXshmfence, Pixman, XcbProto, XcbUtil,
    XkeyboardConfig, XorgFontUtil, XorgProto, XorgUtilMacros, XorgXkbcomp, Xtrans,
};

fn xorg_server_flags() -> Vec<String> {
    vec![
        "-Dxorg=true".to_string(),
        "-Dxv=false".to_string(),
        "-Dxvfb=false".to_string(),
        "-Dxephyr=false".to_string(),
        "-Dxnest=false".to_string(),
        "-Dsuid_wrapper=false".to_string(),
        "-Dpciaccess=false".to_string(),
        "-Ddpms=false".to_string(),
        "-Dscreensaver=false".to_string(),
        "-Dxres=false".to_string(),
        "-Dxvmc=false".to_string(),
        "-Dsystemd_logind=false".to_string(),
        "-Dsecure-rpc=false".to_string(),
        "-Dudev=false".to_string(),
        "-Dudev_kms=false".to_string(),
        "-Ddri1=false".to_string(),
        "-Ddri2=false".to_string(),
        "-Ddri3=false".to_string(),
        "-Dint10=false".to_string(),
        "-Dvgahw=false".to_string(),
        "-Ddrm=false".to_string(),
        "-Dglamor=false".to_string(),
        "-Dglx=false".to_string(),
        "-Dlisten_tcp=false".to_string(),
    ]
}

fn xorg_server_args() -> Vec<String> {
    vec![
        format!("-Dxkb_bin_dir={BINDIR}"),
        format!("-Dxkb_dir={XKB_DIR}"),
        format!("-Dxkb_output_dir={XKB_OUTPUT_DIR}"),
        format!("-Ddefault_font_path={DEFAULT_FONT_PATH}"),
    ]
}

make_package!(
    XorgServer,
    "xorg-server",
    git_url = "https://gitlab.freedesktop.org/xorg/xserver.git",
    git_commit = "312a25c65c8a918fea2cc77abd0db07ec0fc421c",
    dependencies = [
        XorgUtilMacros, XorgProto, XorgFontUtil, XcbProto, XcbUtil, Xtrans, LibXinerama, LibXcvt,
        LibXshmfence, LibX11, LibXkbfile, LibXmu, LibXfont2, LibXi, LibXrender, LibXrandr,
        LibXcb, LibXfixes, LibXext, LibXdamage, Pixman, XorgXkbcomp, XkeyboardConfig
    ],
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            configure_meson(self, ctx, xorg_server_flags(), xorg_server_args())
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            build_meson(self, ctx)
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            install_meson(self, ctx)?;
            xorg_server_install_hook(ctx)
        }
    }
);
