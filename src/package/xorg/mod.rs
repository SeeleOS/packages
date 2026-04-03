pub(crate) mod hooks;
mod meta;
mod simple_autotools;
mod simple_meson;
mod xorg_server;

pub use meta::{GuiPackage, XorgPackage};
pub use simple_autotools::{
    LibFontenc, LibIce, LibSm, LibX11, LibXcb, LibXdamage, LibXdmcp, LibXext, LibXfont2, LibXi,
    LibXmu, LibXrandr, LibXrender, LibXshmfence, LibXt, XcbProto, XcbUtil, XorgFontUtil,
    XorgTwm, XorgUtilMacros, XorgXauth, XorgXinit, XorgXkbcomp, XorgXmodmap, XorgXrdb, Xtrans,
};
pub use simple_meson::{
    Freetype2, LibXau, LibXcvt, LibXfixes, LibXinerama, LibXkbfile, Pixman, XkeyboardConfig,
    XorgProto, XorgXeyes,
};
pub use xorg_server::XorgServer;
