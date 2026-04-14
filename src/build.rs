use crate::{
    command::{CommandSpec, make, run},
    configure::with_envs,
    cross::{meson_vala_wrapper, pkg_env, target_env},
    r#trait::Package,
    types::{Context, Result},
};
use std::path::Path;

pub const CC: &str = "clang --target=x86_64-seele";

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
    let vala = meson_vala_wrapper(ctx, &paths)?;
    let jobs = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    run(pkg_env(
        CommandSpec::new("meson")
            .arg("compile")
            .arg("-C")
            .arg(&paths.build)
            .arg(format!("-j{jobs}"))
            .env("VALAC", &vala)
            .env_prepend("PATH", paths.root.display().to_string(), ":"),
        ctx,
    )?)
}

pub fn build_cmake(pkg: &dyn Package, _ctx: &Context) -> Result<()> {
    let paths = pkg.calc_paths(_ctx);
    let jobs = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    run(CommandSpec::new("cmake")
        .arg("--build")
        .arg(&paths.build)
        .arg("--parallel")
        .arg(jobs.to_string()))
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

pub fn build_cargo(pkg: &dyn Package, ctx: &Context) -> Result<()> {
    build_cargo_with(pkg, ctx, Vec::new(), Vec::new())
}

pub fn build_cargo_with(
    pkg: &dyn Package,
    ctx: &Context,
    envs: Vec<(String, String)>,
    extra_args: Vec<String>,
) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    let mut cmd = with_envs(
        target_env(
            CommandSpec::new("cargo")
                .cwd(&paths.src)
                .arg("+seele")
                .arg("build")
                .arg("--target")
                .arg(crate::cross::TARGET_TRIPLE)
                .arg("--target-dir")
                .arg(paths.build.join("target")),
            ctx,
        )?,
        envs,
    )
    .env("CARGO_TARGET_X86_64_SEELE_LINKER", "clang");

    for arg in extra_args {
        cmd = cmd.arg(arg);
    }

    run(cmd)
}
