mod command;
mod fetch;
mod fs_utils;
mod install;
mod package;
mod r#trait;
mod types;

use std::env;
use std::process;

use package::bash::Bash;
use package::busybox::Busybox;
use package::tinycc::TinyCc;
use r#trait::Package;
use types::{Action, Context, Result};

fn usage() {
    eprintln!("Usage:");
    eprintln!("  pkgs install <package>");
    eprintln!("  pkgs build <package>");
    eprintln!("  pkgs fetch <package>");
    eprintln!("  pkgs patch <package>");
    eprintln!("  pkgs configure <package>");
    eprintln!("  pkgs clean <package>");
    eprintln!("  pkgs list");
}

fn package_by_name(name: &str) -> Option<Box<dyn Package>> {
    match name {
        "bash" => Some(Box::new(Bash)),
        "busybox" => Some(Box::new(Busybox)),
        "tcc" | "tinycc" => Some(Box::new(TinyCc)),
        _ => None,
    }
}

fn action_from_name(name: &str) -> Option<Action> {
    match name {
        "install" => Some(Action::Install),
        "build" => Some(Action::Build),
        "fetch" => Some(Action::Fetch),
        "patch" => Some(Action::Patch),
        "configure" => Some(Action::Configure),
        "clean" => Some(Action::Clean),
        "list" => Some(Action::List),
        _ => None,
    }
}

fn run() -> Result<()> {
    let mut args = env::args().skip(1);
    let Some(action_name) = args.next() else {
        usage();
        process::exit(1);
    };

    let Some(action) = action_from_name(&action_name) else {
        usage();
        process::exit(1);
    };

    if action == Action::List {
        for name in ["bash", "busybox", "tinycc"] {
            println!("{name}");
        }
        return Ok(());
    }

    let Some(pkg_name) = args.next() else {
        usage();
        process::exit(1);
    };

    if args.next().is_some() {
        usage();
        process::exit(1);
    }

    let ctx = Context::discover()?;
    let pkg = package_by_name(&pkg_name).ok_or_else(|| format!("unknown package: {pkg_name}"))?;
    pkg.run(&ctx, action)
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}
