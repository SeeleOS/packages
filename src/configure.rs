use crate::command::{run, CommandSpec};
use crate::cross::{build_triplet, meson_cross_file, pkg_env, target_env, TARGET_TRIPLE};
use crate::fs_utils::ensure_dir;
use crate::gnu_config::refresh_gnu_config;
use crate::layout::{
    BINDIR, INCLUDEDIR, LIBDIR, LOCALSTATEDIR, PREFIX, SBINDIR, SYSCONFDIR, relative_dir,
};
use crate::libtool::fix_libtool_scripts;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub fn with_envs<'a>(
    mut spec: CommandSpec<'a>,
    envs: Vec<(String, String)>,
) -> CommandSpec<'a> {
    for (key, value) in envs {
        spec = spec.env(key, value);
    }
    spec
}

pub fn with_autotools_layout<'a>(spec: CommandSpec<'a>) -> CommandSpec<'a> {
    spec.arg(format!("--prefix={PREFIX}"))
        .arg(format!("--bindir={BINDIR}"))
        .arg(format!("--sbindir={SBINDIR}"))
        .arg(format!("--includedir={INCLUDEDIR}"))
        .arg(format!("--libdir={LIBDIR}"))
        .arg(format!("--sysconfdir={SYSCONFDIR}"))
        .arg(format!("--localstatedir={LOCALSTATEDIR}"))
}

pub fn with_meson_layout<'a>(spec: CommandSpec<'a>) -> CommandSpec<'a> {
    spec.arg(format!("--prefix={PREFIX}"))
        .arg(format!("--bindir={}", relative_dir(BINDIR)))
        .arg(format!("--sbindir={}", relative_dir(SBINDIR)))
        .arg(format!("--includedir={}", relative_dir(INCLUDEDIR)))
        .arg(format!("--libdir={}", relative_dir(LIBDIR)))
        .arg(format!("--sysconfdir={SYSCONFDIR}"))
        .arg(format!("--localstatedir={LOCALSTATEDIR}"))
}

pub fn configure_autotools(
    pkg: &dyn Package,
    ctx: &Context,
    envs: Vec<(String, String)>,
    extra_args: Vec<String>,
    extra_dynamic: Vec<String>,
) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    configure_autotools_in(
        &paths.src,
        with_envs(CommandSpec::new("./configure").cwd(&paths.src), envs),
        ctx,
        extra_args,
        extra_dynamic,
    )
}

pub fn configure_autotools_in<'a>(
    source_dir: &std::path::Path,
    spec: CommandSpec<'a>,
    ctx: &Context,
    extra_args: Vec<String>,
    extra_dynamic: Vec<String>,
) -> Result<()> {
    refresh_gnu_config(ctx, source_dir)?;
    let mut cmd = with_autotools_layout(target_env(spec, ctx)?)
        .arg(format!("--build={}", build_triplet(source_dir)?))
        .arg(format!("--host={TARGET_TRIPLE}"))
        .arg(format!("--target={TARGET_TRIPLE}"))
        .arg("--enable-shared")
        .arg("--disable-static");
    for arg in extra_args {
        cmd = cmd.arg(arg);
    }
    for arg in extra_dynamic {
        cmd = cmd.arg(arg);
    }
    run(cmd)?;
    fix_libtool_scripts(source_dir)
}

pub fn configure_meson(
    pkg: &dyn Package,
    ctx: &Context,
    extra_args: Vec<String>,
    extra_dynamic: Vec<String>,
) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    ensure_dir(&paths.build)?;
    let mut cmd = with_meson_layout(pkg_env(
        CommandSpec::new("meson").arg("setup").cwd(&paths.root),
        ctx,
    )?)
        .arg(&paths.build)
        .arg(&paths.src)
        .arg(format!("--cross-file={}", meson_cross_file(ctx, &paths)?.display()))
        .arg("--buildtype=release")
        .arg("--wrap-mode=nodownload")
        .arg("-Ddefault_library=shared");
    for arg in extra_args {
        cmd = cmd.arg(arg);
    }
    for arg in extra_dynamic {
        cmd = cmd.arg(arg);
    }
    run(cmd)
}
