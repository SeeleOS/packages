mod freetype2;
pub(crate) mod hooks;
mod libxau;
mod libxcvt;
mod libxfixes;
mod libxinerama;
mod libxkbfile;
mod meta;
mod pixman;
mod simple_autotools;
mod xkeyboard_config;
mod xorg_proto;
mod xorg_server;
mod xorg_xeyes;

pub use freetype2::Freetype2;
pub use libxau::LibXau;
pub use libxcvt::LibXcvt;
pub use libxfixes::LibXfixes;
pub use libxinerama::LibXinerama;
pub use libxkbfile::LibXkbfile;
pub use meta::{GuiPackage, XorgPackage};
pub use pixman::Pixman;
pub use simple_autotools::{
    LibFontenc, LibIce, LibSm, LibX11, LibXcb, LibXdamage, LibXdmcp, LibXext, LibXfont2, LibXi,
    LibXmu, LibXrandr, LibXrender, LibXshmfence, LibXt, XcbProto, XcbUtil, XorgFontUtil,
    XorgTwm, XorgUtilMacros, XorgXauth, XorgXinit, XorgXkbcomp, XorgXmodmap, XorgXrdb, Xtrans,
};
pub use xkeyboard_config::XkeyboardConfig;
pub use xorg_proto::XorgProto;
pub use xorg_server::XorgServer;
pub use xorg_xeyes::XorgXeyes;
