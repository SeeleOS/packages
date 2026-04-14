use crate::command::{CommandSpec, run};
use crate::cross::{
    TARGET_TRIPLE, build_triplet, meson_cross_file, meson_native_file, pkg_env, target_env,
};
use crate::fs_utils::ensure_dir;
use crate::gnu_config::refresh_gnu_config;
use crate::layout::{
    BINDIR, INCLUDEDIR, LIB_BINARY_DIR, LOCALSTATEDIR, PREFIX, SBINDIR, SYSCONFDIR, relative_dir,
};
use crate::libtool::fix_libtool_scripts;
use crate::misc::sysroot_dir;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub fn with_envs<'a>(mut spec: CommandSpec<'a>, envs: Vec<(String, String)>) -> CommandSpec<'a> {
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
        .arg(format!("--libdir={LIB_BINARY_DIR}"))
        .arg(format!("--sysconfdir={SYSCONFDIR}"))
        .arg(format!("--localstatedir={LOCALSTATEDIR}"))
}

pub fn with_meson_layout<'a>(spec: CommandSpec<'a>) -> CommandSpec<'a> {
    spec.arg(format!("--prefix={PREFIX}"))
        .arg(format!("--bindir={}", relative_dir(BINDIR)))
        .arg(format!("--sbindir={}", relative_dir(SBINDIR)))
        .arg(format!("--includedir={}", relative_dir(INCLUDEDIR)))
        .arg(format!("--libdir={}", relative_dir(LIB_BINARY_DIR)))
        .arg(format!("--sysconfdir={SYSCONFDIR}"))
        .arg(format!("--localstatedir={LOCALSTATEDIR}"))
}

pub fn with_cmake_layout<'a>(spec: CommandSpec<'a>) -> CommandSpec<'a> {
    spec.arg(format!("-DCMAKE_INSTALL_PREFIX={PREFIX}"))
        .arg(format!("-DCMAKE_INSTALL_BINDIR={BINDIR}"))
        .arg(format!("-DCMAKE_INSTALL_SBINDIR={SBINDIR}"))
        .arg(format!("-DCMAKE_INSTALL_INCLUDEDIR={INCLUDEDIR}"))
        .arg(format!("-DCMAKE_INSTALL_LIBDIR={LIB_BINARY_DIR}"))
        .arg(format!("-DCMAKE_INSTALL_SYSCONFDIR={SYSCONFDIR}"))
        .arg(format!("-DCMAKE_INSTALL_LOCALSTATEDIR={LOCALSTATEDIR}"))
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
    .arg(format!(
        "--cross-file={}",
        meson_cross_file(ctx, &paths)?.display()
    ))
    // Keep Meson's build-machine lookups separate from target pkg-config.
    .arg(format!(
        "--native-file={}",
        meson_native_file(ctx, &paths)?.display()
    ))
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

pub fn configure_cmake(
    pkg: &dyn Package,
    ctx: &Context,
    extra_args: Vec<String>,
    extra_dynamic: Vec<String>,
) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    let sysroot = sysroot_dir(ctx)?;
    ensure_dir(&paths.build)?;
    let mut cmd = with_cmake_layout(pkg_env(CommandSpec::new("cmake").cwd(&paths.root), ctx)?)
        .arg("-S")
        .arg(&paths.src)
        .arg("-B")
        .arg(&paths.build)
        .arg("-G")
        .arg("Ninja")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DCMAKE_C_COMPILER=clang")
        .arg("-DCMAKE_CXX_COMPILER=clang++")
        .arg(format!("-DCMAKE_C_COMPILER_TARGET={TARGET_TRIPLE}"))
        .arg(format!("-DCMAKE_CXX_COMPILER_TARGET={TARGET_TRIPLE}"))
        .arg(format!("-DCMAKE_ASM_COMPILER_TARGET={TARGET_TRIPLE}"))
        .arg(format!("-DCMAKE_SYSROOT={}", sysroot.display()))
        .arg("-DCMAKE_POSITION_INDEPENDENT_CODE=ON")
        .arg(format!(
            "-DCMAKE_C_FLAGS=-fPIC -I{} -I{}",
            ctx.include_root_dir.display(),
            ctx.include_c_dir.display()
        ))
        .arg(format!(
            "-DCMAKE_CXX_FLAGS=-fPIC -I{} -I{}",
            ctx.include_root_dir.display(),
            ctx.include_c_dir.display()
        ))
        .arg(format!(
            "-DCMAKE_EXE_LINKER_FLAGS=-L{} -Wl,-rpath-link,{}",
            ctx.lib_binary_dir.display(),
            ctx.lib_binary_dir.display()
        ))
        .arg(format!(
            "-DCMAKE_SHARED_LINKER_FLAGS=-L{} -Wl,-rpath-link,{}",
            ctx.lib_binary_dir.display(),
            ctx.lib_binary_dir.display()
        ))
        .arg("-DCMAKE_TRY_COMPILE_TARGET_TYPE=STATIC_LIBRARY");
    for arg in extra_args {
        cmd = cmd.arg(arg);
    }
    for arg in extra_dynamic {
        cmd = cmd.arg(arg);
    }
    run(cmd)
}
