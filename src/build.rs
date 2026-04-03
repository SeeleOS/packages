use crate::{
    command::{make, run, CommandSpec},
    cross::{pkg_env, target_env},
    r#trait::Package,
    configure::with_envs,
    types::{Context, Result},
};
use std::path::Path;

pub const CC: &str = "clang --target=x86_64-seele";

pub fn build_relibc(ctx: &Context) -> Result<()> {
    run(make()
        .cwd(&ctx.relibc_root)
        .env_remove("CARGO")
        .env_remove("CARGO_MANIFEST_DIR")
        .env_remove("CARGO_MANIFEST_PATH")
        .env_remove("RUSTUP_TOOLCHAIN")
        .env_remove("RUST_RECURSION_COUNT")
        .arg("all"))
}

pub fn build_autotools_with(
    pkg: &dyn Package,
    ctx: &Context,
    envs: Vec<(String, String)>,
    extra_args: Vec<String>,
) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    let mut cmd = with_envs(target_env(make().cwd(&paths.src), ctx)?, envs);
    for arg in extra_args {
        cmd = cmd.arg(arg);
    }
    run(cmd)
}

pub fn build_meson(pkg: &dyn Package, ctx: &Context) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    let jobs = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    run(pkg_env(
        CommandSpec::new("meson")
            .arg("compile")
            .arg("-C")
            .arg(&paths.build)
            .arg(format!("-j{jobs}")),
        ctx,
    )?)
}

pub fn build_make_in(
    cwd: &Path,
    envs: Vec<(String, String)>,
    extra_args: Vec<String>,
) -> Result<()> {
    let mut cmd = with_envs(make().cwd(cwd), envs);
    for arg in extra_args {
        cmd = cmd.arg(arg);
    }
    run(cmd)
}
