use crate::layout::APPDEFAULTDIR;
use crate::make_autotools_packages;
use crate::package::xorg::{Freetype2, LibXau, LibXfixes, LibXkbfile, XorgProto};

const LIBXFONT2_ARGS: &[&str] = &["--disable-devel-docs", "--disable-selective-werror"];

fn libx11_extra_args(ctx: &crate::types::Context) -> Vec<String> {
    vec![format!(
        "--with-keysymdefdir={}",
        ctx.system_include_dir.join("X11").display()
    )]
}

fn libxt_args() -> Vec<String> {
    vec![format!("--with-appdefaultdir={APPDEFAULTDIR}")]
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
    { LibX11, "libx11", tarball_url = "https://www.x.org/archive/individual/lib/libX11-1.8.13.tar.xz", dependencies = [XorgProto, LibXcb, Xtrans], configure = { dynamic_args = libx11_extra_args } },
    { LibXfont2, "libxfont2", tarball_url = "https://www.x.org/archive/individual/lib/libXfont2-2.0.7.tar.gz", dependencies = [XorgUtilMacros, XorgProto, LibX11, Xtrans, Freetype2, LibFontenc], configure = { args = LIBXFONT2_ARGS } },
    { LibXmu, "libxmu", tarball_url = "https://www.x.org/archive/individual/lib/libXmu-1.3.1.tar.gz", dependencies = [LibXext, LibXt] },
    { LibXrandr, "libxrandr", tarball_url = "https://www.x.org/archive/individual/lib/libXrandr-1.5.5.tar.gz", dependencies = [XorgProto, LibX11, LibXrender, LibXext] },
    { LibXrender, "libxrender", tarball_url = "https://www.x.org/archive/individual/lib/libXrender-0.9.12.tar.gz", dependencies = [XorgProto, LibX11] },
    { LibXshmfence, "libxshmfence", tarball_url = "https://www.x.org/archive/individual/lib/libxshmfence-1.3.3.tar.gz", dependencies = [XorgProto] },
    { LibXt, "libxt", tarball_url = "https://www.x.org/archive/individual/lib/libXt-1.3.1.tar.gz", dependencies = [LibX11, LibSm], configure = { dynamic_args = |_| libxt_args() } },
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
