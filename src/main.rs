mod build;
mod command;
mod fetch;
mod fs_utils;
mod install;
mod meta_pkg;
mod misc;
mod package;
mod trace;
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
use crate::trace::{detail, section};

fn usage() {
    eprintln!("Usage:");
    eprintln!("  cargo run install <package>");
    eprintln!("  cargo run clean <package>");
}

fn package_by_name(name: &str) -> Option<Box<dyn Package>> {
    match name {
        "bash" => Some(Box::new(Bash)),
        "busybox" => Some(Box::new(Busybox)),
        "ncurses" => Some(Box::new(Ncurses)),
        "tcc" | "tinycc" => Some(Box::new(TinyCc)),
        "vim" => Some(Box::new(Vim)),
        "base" => Some(Box::new(BasePackage)),
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

    let Some(pkg_name) = args.next() else {
        usage();
        process::exit(1);
    };

    if args.next().is_some() {
        usage();
        process::exit(1);
    }

    section(format!(
        "starting package action: action=`{}` package=`{}`",
        action_name, pkg_name
    ));
    let ctx = Context::discover()?;
    detail(format!(
        "resolved context: packages_root={} install_dir={} relibc_root={}",
        ctx.packages_root.display(),
        ctx.install_dir.display(),
        ctx.relibc_root.display()
    ));
    let pkg = package_by_name(&pkg_name).ok_or_else(|| format!("unknown package: {pkg_name}"))?;
    pkg.run(&ctx, action)
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}
