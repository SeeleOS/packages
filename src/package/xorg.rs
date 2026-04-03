use crate::build::build_meson;
use crate::configure::configure_meson;
use crate::fs_utils::copy_file;
use crate::install::install_meson;
use crate::layout::{APPDEFAULTDIR, BINDIR, DEFAULT_FONT_PATH, XKB_DIR, XKB_OUTPUT_DIR};
use crate::make_autotools_packages;
use crate::make_meson_packages;
use crate::make_meta_package;
use crate::make_package;
use crate::misc::sysroot_dir;

fn xorg_server_install_hook(ctx: &crate::types::Context) -> crate::types::Result<()> {
    let source = ctx.packages_root.join("xorg-server/xorg.conf");
    let target = sysroot_dir(ctx)?.join("etc/X11/xorg.conf");
    copy_file(&source, &target)
}

make_autotools_packages!(
    { LibFontenc, "libfontenc", tarball_url = "https://www.x.org/archive/individual/lib/libfontenc-1.1.9.tar.gz", dependencies = [XorgProto] },
    { LibIce, "libice", tarball_url = "https://www.x.org/archive/individual/lib/libICE-1.1.2.tar.gz", dependencies = [XorgProto, Xtrans] },
    { LibSm, "libsm", tarball_url = "https://www.x.org/archive/individual/lib/libSM-1.2.6.tar.gz", dependencies = [XorgProto, LibIce] },
    { LibXcb, "libxcb", tarball_url = "https://www.x.org/archive/individual/lib/libxcb-1.17.0.tar.xz", dependencies = [XorgProto, LibXau, LibXdmcp, XcbProto] },
    { LibXdamage, "libxdamage", tarball_url = "https://www.x.org/archive/individual/lib/libXdamage-1.1.7.tar.gz", dependencies = [XorgProto, LibX11, LibXfixes] },
    { LibXdmcp, "libxdmcp", tarball_url = "https://www.x.org/archive/individual/lib/libXdmcp-1.1.5.tar.gz", dependencies = [XorgProto] },
    { LibXext, "libxext", tarball_url = "https://www.x.org/archive/individual/lib/libXext-1.3.7.tar.gz", dependencies = [XorgProto, LibX11] },
    { LibXi, "libxi", tarball_url = "https://www.x.org/archive/individual/lib/libXi-1.8.2.tar.gz", dependencies = [XorgProto, LibXext, LibXfixes] },
    { LibX11, "libx11", tarball_url = "https://www.x.org/archive/individual/lib/libX11-1.8.13.tar.xz", dependencies = [XorgProto, LibXcb, Xtrans], configure = { dynamic_args = |ctx: &crate::types::Context| vec![format!("--with-keysymdefdir={}", ctx.system_include_dir.join("X11").display())] } },
    { LibXfont2, "libxfont2", tarball_url = "https://www.x.org/archive/individual/lib/libXfont2-2.0.7.tar.gz", dependencies = [XorgUtilMacros, XorgProto, LibX11, Xtrans, Freetype2, LibFontenc], configure = { args = vec!["--disable-devel-docs".to_string(), "--disable-selective-werror".to_string()] } },
    { LibXmu, "libxmu", tarball_url = "https://www.x.org/archive/individual/lib/libXmu-1.3.1.tar.gz", dependencies = [LibXext, LibXt] },
    { LibXrandr, "libxrandr", tarball_url = "https://www.x.org/archive/individual/lib/libXrandr-1.5.5.tar.gz", dependencies = [XorgProto, LibX11, LibXrender, LibXext] },
    { LibXrender, "libxrender", tarball_url = "https://www.x.org/archive/individual/lib/libXrender-0.9.12.tar.gz", dependencies = [XorgProto, LibX11] },
    { LibXshmfence, "libxshmfence", tarball_url = "https://www.x.org/archive/individual/lib/libxshmfence-1.3.3.tar.gz", dependencies = [XorgProto] },
    { LibXt, "libxt", tarball_url = "https://www.x.org/archive/individual/lib/libXt-1.3.1.tar.gz", dependencies = [LibX11, LibSm], configure = { dynamic_args = |_| vec![format!("--with-appdefaultdir={APPDEFAULTDIR}")] } },
    { XcbProto, "xcb-proto", tarball_url = "https://www.x.org/archive/individual/proto/xcb-proto-1.17.0.tar.xz" },
    { XcbUtil, "xcb-util", tarball_url = "https://xcb.freedesktop.org/dist/xcb-util-0.4.1.tar.xz", dependencies = [LibXcb] },
    { XorgFontUtil, "xorg-font-util", tarball_url = "https://www.x.org/archive/individual/font/font-util-1.4.1.tar.xz", dependencies = [XorgUtilMacros] },
    { XorgTwm, "xorg-twm", tarball_url = "https://www.x.org/pub/individual/app/twm-1.0.13.1.tar.xz", dependencies = [LibXmu] },
    { XorgUtilMacros, "xorg-util-macros", tarball_url = "https://www.x.org/archive/individual/util/util-macros-1.20.2.tar.gz" },
    { XorgXauth, "xorg-xauth", tarball_url = "https://www.x.org/releases/individual/app/xauth-1.1.5.tar.xz", dependencies = [LibXmu, LibXau, LibXext, LibX11] },
    { XorgXinit, "xorg-xinit", tarball_url = "https://www.x.org/releases/individual/app/xinit-1.4.4.tar.xz", dependencies = [LibX11, XorgXauth, XorgXmodmap, XorgXrdb] },
    { XorgXkbcomp, "xorg-xkbcomp", tarball_url = "https://www.x.org/archive/individual/app/xkbcomp-1.5.0.tar.gz", dependencies = [LibXkbfile, LibX11] },
    { XorgXmodmap, "xorg-xmodmap", tarball_url = "https://www.x.org/releases/individual/app/xmodmap-1.0.11.tar.xz", dependencies = [LibX11] },
    { XorgXrdb, "xorg-xrdb", tarball_url = "https://www.x.org/releases/individual/app/xrdb-1.2.2.tar.xz", dependencies = [LibX11, LibXmu] },
    { Xtrans, "xtrans", tarball_url = "https://www.x.org/archive/individual/lib/xtrans-1.6.0.tar.gz", dependencies = [XorgUtilMacros] },
);

make_meson_packages!(
    { Freetype2, "freetype2", git_url = "https://gitlab.freedesktop.org/freetype/freetype.git", git_commit = "0a0221a1347e2f1e07c395263540026e9a0aa7c7", configure = { args = vec!["-Dbrotli=disabled".to_string(), "-Dbzip2=disabled".to_string(), "-Dharfbuzz=disabled".to_string(), "-Dpng=disabled".to_string(), "-Dzlib=disabled".to_string()] } },
    { Pixman, "pixman", git_url = "https://gitlab.freedesktop.org/pixman/pixman.git", git_commit = "9cc163c9da0fb4da430641715313d95a6ec466d9", configure = { args = vec!["-Dgtk=disabled".to_string(), "-Dlibpng=disabled".to_string(), "-Dtests=disabled".to_string()] } },
    { LibXau, "libxau", tarball_url = "https://www.x.org/archive/individual/lib/libXau-1.0.12.tar.gz", dependencies = [XorgProto] },
    { LibXfixes, "libxfixes", git_url = "https://gitlab.freedesktop.org/xorg/lib/libxfixes.git", git_commit = "70d5b0e37f8a759f3dbc218f22954347ceed094a", dependencies = [XorgProto, LibX11] },
    { LibXinerama, "libxinerama", tarball_url = "https://www.x.org/archive/individual/lib/libXinerama-1.1.6.tar.gz", dependencies = [LibX11, LibXext, XorgUtilMacros, XorgProto] },
    { LibXkbfile, "libxkbfile", tarball_url = "https://www.x.org/archive/individual/lib/libxkbfile-1.2.0.tar.xz", dependencies = [XorgUtilMacros, XorgProto, LibX11] },
    { LibXcvt, "libxcvt", tarball_url = "https://www.x.org/archive/individual/lib/libxcvt-0.1.3.tar.xz" },
    { XkeyboardConfig, "xkeyboard-config", git_url = "https://gitlab.freedesktop.org/xkeyboard-config/xkeyboard-config.git", git_commit = "a79055334104d382bd511760b67acf9a5a161361", dependencies = [XorgXkbcomp], configure = { args = vec!["-Dxorg-rules-symlinks=true".to_string()] } },
    { XorgProto, "xorg-proto", tarball_url = "https://www.x.org/releases/individual/proto/xorgproto-2025.1.tar.xz", dependencies = [XorgUtilMacros] },
    { XorgXeyes, "xorg-xeyes", git_url = "https://gitlab.freedesktop.org/xorg/app/xeyes.git", git_commit = "7dc4f720f57471d2eccefd87cdca54494cf75eb5", dependencies = [LibX11, LibXt, LibXext, LibXmu, LibXrender, LibXi, LibXcb] },
);

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
            configure_meson(
                self,
                ctx,
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
                ],
                vec![
                    format!("-Dxkb_bin_dir={BINDIR}"),
                    format!("-Dxkb_dir={XKB_DIR}"),
                    format!("-Dxkb_output_dir={XKB_OUTPUT_DIR}"),
                    format!("-Ddefault_font_path={DEFAULT_FONT_PATH}"),
                ],
            )
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

pub struct GuiPackage;
make_meta_package!("gui", GuiPackage, XorgServer, XorgXinit, XorgTwm, XorgXeyes);

pub struct XorgPackage;
make_meta_package!("xorg", XorgPackage, GuiPackage);
