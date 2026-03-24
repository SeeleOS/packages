use crate::{
    command::{make, run},
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
