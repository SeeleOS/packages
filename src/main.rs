mod build;
mod command;
mod configure;
mod cross;
mod fetch;
mod fs_utils;
mod gnu_config;
mod install;
mod layout;
mod libtool;
mod meta_pkg;
mod misc;
mod make_pkg;
mod package;
mod r#trait;
mod types;

use std::env;
use std::process;

use package::bash::Bash;
use package::busybox::Busybox;
use package::ncurses::Ncurses;
use package::tinycc::TinyCc;
use package::vim::Vim;
use r#trait::Package;
use types::{Action, Context, Result};

use crate::package::base::BasePackage;
use crate::package::xorg;

fn usage() {
    eprintln!("Usage:");
    eprintln!("  cargo run install <package> [--rebuild] [--ignore-deps]");
    eprintln!("  cargo run deploy <package> [--rebuild] [--ignore-deps]");
    eprintln!("  cargo run clean <package> [--rebuild] [--ignore-deps]");
}

fn package_by_name(name: &str) -> Option<Box<dyn Package>> {
    match name {
        "bash" => Some(Box::new(Bash)),
        "busybox" => Some(Box::new(Busybox)),
        "ncurses" => Some(Box::new(Ncurses)),
        "tcc" | "tinycc" => Some(Box::new(TinyCc)),
        "vim" => Some(Box::new(Vim)),
        "base" => Some(Box::new(BasePackage)),
        "gui" => Some(Box::new(xorg::GuiPackage)),
        "xorg" => Some(Box::new(xorg::XorgPackage)),
        "xcb-proto" => Some(Box::new(xorg::XcbProto)),
        "xorg-proto" => Some(Box::new(xorg::XorgProto)),
        "xorg-util-macros" => Some(Box::new(xorg::XorgUtilMacros)),
        "xtrans" => Some(Box::new(xorg::Xtrans)),
        "libx11" => Some(Box::new(xorg::LibX11)),
        "libxau" => Some(Box::new(xorg::LibXau)),
        "libxcb" => Some(Box::new(xorg::LibXcb)),
        "libxdmcp" => Some(Box::new(xorg::LibXdmcp)),
        "libxext" => Some(Box::new(xorg::LibXext)),
        "libxdamage" => Some(Box::new(xorg::LibXdamage)),
        "libxfixes" => Some(Box::new(xorg::LibXfixes)),
        "libxi" => Some(Box::new(xorg::LibXi)),
        "libxrandr" => Some(Box::new(xorg::LibXrandr)),
        "libxrender" => Some(Box::new(xorg::LibXrender)),
        "libice" => Some(Box::new(xorg::LibIce)),
        "libsm" => Some(Box::new(xorg::LibSm)),
        "libxinerama" => Some(Box::new(xorg::LibXinerama)),
        "libxmu" => Some(Box::new(xorg::LibXmu)),
        "libxt" => Some(Box::new(xorg::LibXt)),
        "freetype" | "freetype2" => Some(Box::new(xorg::Freetype2)),
        "libfontenc" => Some(Box::new(xorg::LibFontenc)),
        "libxcvt" => Some(Box::new(xorg::LibXcvt)),
        "libxfont2" => Some(Box::new(xorg::LibXfont2)),
        "pixman" => Some(Box::new(xorg::Pixman)),
        "libxkbfile" => Some(Box::new(xorg::LibXkbfile)),
        "libxshmfence" => Some(Box::new(xorg::LibXshmfence)),
        "xcb-util" => Some(Box::new(xorg::XcbUtil)),
        "xkeyboard-config" => Some(Box::new(xorg::XkeyboardConfig)),
        "xorg-font-util" => Some(Box::new(xorg::XorgFontUtil)),
        "xorg-server" => Some(Box::new(xorg::XorgServer)),
        "xorg-xkbcomp" | "xkbcomp" => Some(Box::new(xorg::XorgXkbcomp)),
        "xorg-twm" | "twm" => Some(Box::new(xorg::XorgTwm)),
        "xorg-xauth" | "xauth" => Some(Box::new(xorg::XorgXauth)),
        "xorg-xeyes" | "xeyes" => Some(Box::new(xorg::XorgXeyes)),
        "xorg-xinit" | "xinit" => Some(Box::new(xorg::XorgXinit)),
        "xorg-xmodmap" | "xmodmap" => Some(Box::new(xorg::XorgXmodmap)),
        "xorg-xrdb" | "xrdb" => Some(Box::new(xorg::XorgXrdb)),
        _ => None,
    }
}

fn run() -> Result<()> {
    let mut args = env::args().skip(1);
    let Some(action_name) = args.next() else {
        usage();
        process::exit(1);
    };

    let Some(action) = Action::from_str(&action_name) else {
        usage();
        process::exit(1);
    };

    let mut pkg_name = None;
    let mut rebuild = false;
    let mut ignore_deps = false;
    for arg in args {
        if arg == "--rebuild" {
            rebuild = true;
            continue;
        }
        if arg == "--ignore-deps" {
            ignore_deps = true;
            continue;
        }
        if pkg_name.is_none() {
            pkg_name = Some(arg);
            continue;
        }
        usage();
        process::exit(1);
    }

    let Some(pkg_name) = pkg_name else {
        usage();
        process::exit(1);
    };

    let ctx = Context::discover(rebuild, ignore_deps)?;
    let pkg = package_by_name(&pkg_name).ok_or_else(|| format!("unknown package: {pkg_name}"))?;
    pkg.run(&ctx, action)
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}
