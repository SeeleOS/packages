use crate::make_meson_packages;
use crate::package::xorg::{
    LibX11, LibXcb, LibXext, LibXi, LibXmu, LibXrender, LibXt, XorgUtilMacros, XorgXkbcomp,
};

fn freetype_args() -> Vec<String> {
    vec![
        "-Dbrotli=disabled".to_string(),
        "-Dbzip2=disabled".to_string(),
        "-Dharfbuzz=disabled".to_string(),
        "-Dpng=disabled".to_string(),
        "-Dzlib=disabled".to_string(),
    ]
}

fn pixman_args() -> Vec<String> {
    vec![
        "-Dgtk=disabled".to_string(),
        "-Dlibpng=disabled".to_string(),
        "-Dtests=disabled".to_string(),
    ]
}

fn xkeyboard_config_args() -> Vec<String> {
    vec!["-Dxorg-rules-symlinks=true".to_string()]
}

make_meson_packages!(
    { Freetype2, "freetype2", git_url = "https://gitlab.freedesktop.org/freetype/freetype.git", git_commit = "0a0221a1347e2f1e07c395263540026e9a0aa7c7", configure = { args = freetype_args() } },
    { Pixman, "pixman", git_url = "https://gitlab.freedesktop.org/pixman/pixman.git", git_commit = "9cc163c9da0fb4da430641715313d95a6ec466d9", configure = { args = pixman_args() } },
    { LibXau, "libxau", tarball_url = "https://www.x.org/archive/individual/lib/libXau-1.0.12.tar.gz", dependencies = [XorgProto] },
    { LibXfixes, "libxfixes", git_url = "https://gitlab.freedesktop.org/xorg/lib/libxfixes.git", git_commit = "70d5b0e37f8a759f3dbc218f22954347ceed094a", dependencies = [XorgProto, LibX11] },
    { LibXinerama, "libxinerama", tarball_url = "https://www.x.org/archive/individual/lib/libXinerama-1.1.6.tar.gz", dependencies = [LibX11, LibXext, XorgUtilMacros, XorgProto] },
    { LibXkbfile, "libxkbfile", tarball_url = "https://www.x.org/archive/individual/lib/libxkbfile-1.2.0.tar.xz", dependencies = [XorgUtilMacros, XorgProto, LibX11] },
    { LibXcvt, "libxcvt", tarball_url = "https://www.x.org/archive/individual/lib/libxcvt-0.1.3.tar.xz" },
    { XkeyboardConfig, "xkeyboard-config", git_url = "https://gitlab.freedesktop.org/xkeyboard-config/xkeyboard-config.git", git_commit = "a79055334104d382bd511760b67acf9a5a161361", dependencies = [XorgXkbcomp], configure = { args = xkeyboard_config_args() } },
    { XorgProto, "xorg-proto", tarball_url = "https://www.x.org/releases/individual/proto/xorgproto-2025.1.tar.xz", dependencies = [XorgUtilMacros] },
    { XorgXeyes, "xorg-xeyes", git_url = "https://gitlab.freedesktop.org/xorg/app/xeyes.git", git_commit = "7dc4f720f57471d2eccefd87cdca54494cf75eb5", dependencies = [LibX11, LibXt, LibXext, LibXmu, LibXrender, LibXi, LibXcb] },
);
