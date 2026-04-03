use crate::{
    command::{make, run, CommandSpec},
    cross::{pkg_env, target_env},
    r#trait::Package,
    trace::section,
    types::{Context, Result},
};

pub const CC: &str = "clang --target=x86_64-seele";

pub fn build_relibc(ctx: &Context) -> Result<()> {
    section(format!("building relibc in {}", ctx.relibc_root.display()));
    run(make()
        .cwd(&ctx.relibc_root)
        .env_remove("CARGO")
        .env_remove("CARGO_MANIFEST_DIR")
        .env_remove("CARGO_MANIFEST_PATH")
        .env_remove("RUSTUP_TOOLCHAIN")
        .env_remove("RUST_RECURSION_COUNT")
        .arg("all"))
}

pub fn build_autotools(pkg: &dyn Package, ctx: &Context) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    run(target_env(make().cwd(&paths.src), ctx)?)
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
