use crate::{
    command::{CommandSpec, run},
    types::{Context, Result},
};

pub const CC: &str = "clang --target=x86_64-seele";

pub fn build_relibc(ctx: &Context) -> Result<()> {
    run(CommandSpec::new("make")
        .arg("-C")
        .arg(&ctx.relibc_root)
        .arg("all"))
}
