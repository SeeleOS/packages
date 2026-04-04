pub const PREFIX: &str = "/";
pub const BINDIR: &str = "/programs";
pub const SBINDIR: &str = "/programs";
pub const INCLUDEDIR: &str = "/libs/include";
pub const LIB_BINARY_DIR: &str = "/libs/lib_binaries";
pub const SYSCONFDIR: &str = "/etc";
pub const LOCALSTATEDIR: &str = "/var";

pub const APPDEFAULTDIR: &str = "/etc/X11/app-defaults";
pub const XKB_DIR: &str = "/share/X11/xkb";
pub const XKB_OUTPUT_DIR: &str = "/var/lib/xkb";
pub const DEFAULT_FONT_PATH: &str = "/share/fonts/X11";

pub fn relative_dir(path: &'static str) -> &'static str {
    path.strip_prefix('/').unwrap_or(path)
}
