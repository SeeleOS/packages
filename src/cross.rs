use std::fs;
use std::env;
use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::command::{CommandSpec, capture};
use crate::misc::sysroot_dir;
use crate::types::{Context, PackagePaths, Result};

pub const TARGET_TRIPLE: &str = "x86_64-seele";

const TARGET_CC: &str = "clang";
const TARGET_CXX: &str = "clang++";
const TARGET_AR: &str = "llvm-ar";
const TARGET_NM: &str = "llvm-nm";
const TARGET_CARGO: &str = "cargo";
const TARGET_RUST: &str = "rustc";
const TARGET_STRIP: &str = "llvm-strip";

fn meson_array(values: &[String]) -> String {
    let quoted: Vec<String> = values
        .iter()
        .map(|value| format!("'{}'", value.replace('\\', "\\\\").replace('\'', "\\'")))
        .collect();
    format!("[{}]", quoted.join(", "))
}

fn existing_dir_args(flag: &str, dirs: impl IntoIterator<Item = PathBuf>) -> Vec<String> {
    let mut out = Vec::new();
    for dir in dirs {
        if dir.is_dir() {
            out.push(format!("{flag}={}", dir.display()));
        }
    }
    out
}

fn meson_vala_args(ctx: &Context) -> Vec<String> {
    let mut args = Vec::new();
    args.extend(existing_dir_args(
        "--vapidir",
        [
            ctx.staging_sysroot_dir.join("share/vala/vapi"),
            ctx.staging_sysroot_dir.join("libs/lib_binaries/vapi"),
        ],
    ));

    let xdg_dirs: Vec<PathBuf> = match env::var_os("XDG_DATA_DIRS") {
        Some(paths) => env::split_paths(&paths).collect(),
        None => Vec::new(),
    };
    args.extend(existing_dir_args(
        "--vapidir",
        xdg_dirs.iter().map(|dir| dir.join("vala/vapi")),
    ));
    args.extend(existing_dir_args(
        "--girdir",
        xdg_dirs.iter().map(|dir| dir.join("gir-1.0")),
    ));
    args
}

pub fn meson_vala_wrapper(ctx: &Context, paths: &PackagePaths) -> Result<PathBuf> {
    let wrapper = paths.root.join("valac");
    let args = meson_vala_args(ctx);
    let original_path = env::var("PATH").unwrap_or_default().replace('\'', "'\"'\"'");
    let mut script = format!("#!/bin/sh\nexec env PATH='{original_path}' valac");
    for arg in args {
        script.push(' ');
        script.push('\'');
        script.push_str(&arg.replace('\'', "'\"'\"'"));
        script.push('\'');
    }
    script.push_str(" \"$@\"\n");
    fs::write(&wrapper, script)?;
    #[cfg(unix)]
    fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o755))?;
    Ok(wrapper)
}

pub fn build_triplet(source_dir: &Path) -> Result<String> {
    if source_dir.join("config.guess").is_file() {
        Ok(capture(CommandSpec::new("./config.guess").cwd(source_dir))?
            .trim()
            .to_string())
    } else {
        Ok("x86_64-pc-linux-gnu".to_string())
    }
}

pub fn pkg_env<'a>(spec: CommandSpec<'a>, ctx: &'a Context) -> Result<CommandSpec<'a>> {
    let compat_libdir = ctx.staging_sysroot_dir.join("libs").join("pkgconfig");
    let libdir = ctx.lib_binary_dir.join("pkgconfig");
    let sharedir = ctx.staging_sysroot_dir.join("share").join("pkgconfig");
    let staging_share = ctx.staging_sysroot_dir.join("share");
    let staging_vapi = ctx.staging_sysroot_dir.join("share/vala/vapi");
    let llvm_bin = ctx
        .packages_root
        .parent()
        .ok_or("packages directory has no parent")?
        .join(".llvm/bin");
    let current_path = std::env::var_os("PATH").unwrap_or_default();
    let pkg_config_path = format!(
        "{}:{}:{}",
        compat_libdir.display(),
        libdir.display(),
        sharedir.display()
    );
    Ok(spec
        .env_remove("AR")
        .env_remove("CC")
        .env_remove("CPP")
        .env_remove("CXX")
        .env_remove("LD")
        .env_remove("NM")
        .env_remove("RANLIB")
        .env_remove("STRIP")
        .env("RUSTUP_TOOLCHAIN", "seele")
        .env_prepend("PATH", llvm_bin.display().to_string(), ":")
        .env_append("PATH", current_path, ":")
        .env("PKG_CONFIG_ALLOW_CROSS", "1")
        .env("PKG_CONFIG_SYSROOT_DIR", sysroot_dir(ctx)?)
        .env("PKG_CONFIG_LIBDIR", &pkg_config_path)
        .env("PKG_CONFIG_PATH", &pkg_config_path)
        // Meson/Vala host tools still need to see target-installed metadata
        // such as staged VAPI files under share/vala/vapi.
        .env_append("XDG_DATA_DIRS", staging_share.display().to_string(), ":")
        .env_append("VAPIDIR", staging_vapi.display().to_string(), ":")
        .env("PKG_CONFIG_PATH_FOR_TARGET", ""))
}

pub fn target_env<'a>(spec: CommandSpec<'a>, ctx: &'a Context) -> Result<CommandSpec<'a>> {
    Ok(pkg_env(spec, ctx)?
        .env("CC", format!("clang --target={TARGET_TRIPLE}"))
        .env("CXX", format!("clang++ --target={TARGET_TRIPLE}"))
        .env("AR", TARGET_AR)
        .env("NM", TARGET_NM)
        .env("RANLIB", "llvm-ranlib")
        .env("STRIP", "llvm-strip")
        .env_append(
            "CPPFLAGS",
            format!(
                "-I{} -I{}",
                ctx.include_root_dir.display(),
                ctx.include_c_dir.display()
            ),
            " ",
        )
        .env_append("CFLAGS", "-fPIC", " ")
        .env_append(
            "CXXFLAGS",
            format!(
                "-fPIC -idirafter{} -idirafter{}",
                ctx.include_root_dir.display(),
                ctx.include_c_dir.display()
            ),
            " ",
        )
        .env_append(
            "LDFLAGS",
            format!(
                "-L{} -Wl,-rpath-link,{} -lunwind",
                ctx.lib_binary_dir.display(),
                ctx.lib_binary_dir.display()
            ),
            " ",
        )
    )
}

pub fn meson_cross_file(ctx: &Context, paths: &PackagePaths) -> Result<PathBuf> {
    let cross_file = paths.root.join("seele.cross");
    let vala = meson_vala_wrapper(ctx, paths)?;
    fs::write(
        &cross_file,
        format!(
            "[binaries]\n\
             c = ['{TARGET_CC}', '--target={TARGET_TRIPLE}']\n\
             cpp = ['{TARGET_CXX}', '--target={TARGET_TRIPLE}']\n\
             vala = '{vala}'\n\
             cargo = '{TARGET_CARGO}'\n\
             ar = '{TARGET_AR}'\n\
             nm = '{TARGET_NM}'\n\
             rust = ['{TARGET_RUST}', '--target={TARGET_TRIPLE}']\n\
             strip = '{TARGET_STRIP}'\n\
             pkg-config = 'pkg-config'\n\
             \n[built-in options]\n\
             c_args = ['-fPIC', '-I{root_inc}', '-I{c_inc}']\n\
             cpp_args = ['-fPIC', '-idirafter{root_inc}', '-idirafter{c_inc}']\n\
             c_link_args = ['-L{lib}', '-Wl,-rpath-link,{lib}', '-lunwind']\n\
             cpp_link_args = ['-L{lib}', '-Wl,-rpath-link,{lib}', '-lunwind']\n\
             \n[properties]\nneeds_exe_wrapper = true\n\
             \n[host_machine]\n\
             system = 'seele'\ncpu_family = 'x86_64'\ncpu = 'x86_64'\nendian = 'little'\n",
            root_inc = ctx.include_root_dir.display(),
            c_inc = ctx.include_c_dir.display(),
            lib = ctx.lib_binary_dir.display(),
            vala = vala.display(),
        ),
    )?;
    Ok(cross_file)
}

/// Create a build-machine `pkg-config` wrapper for Meson's `native: true`
/// lookups.
///
/// Our normal cross environment points `pkg-config` at the target sysroot so
/// target libraries resolve correctly. Meson uses the native file for build
/// machine tools and dependencies, so reusing that cross `pkg-config`
/// environment would incorrectly resolve host-side tools against target `.pc`
/// files.
pub fn meson_native_pkg_config(ctx: &Context, paths: &PackagePaths) -> Result<PathBuf> {
    let wrapper = paths.root.join("build-pkg-config");
    let path = std::env::var("PATH").unwrap_or_default();
    let pkg_config_path = std::env::var("PKG_CONFIG_PATH").unwrap_or_default();
    let pkg_config_libdir = std::env::var("PKG_CONFIG_LIBDIR").unwrap_or_default();
    let pkg_config_path_for_target =
        std::env::var("PKG_CONFIG_PATH_FOR_TARGET").unwrap_or_default();
    let pkg_config_sysroot_dir = std::env::var("PKG_CONFIG_SYSROOT_DIR").unwrap_or_default();
    fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\n\
# Meson `native: true` lookups must resolve against the build machine,
# not the target sysroot pkg-config environment we export for cross builds.\n\
exec env \\\n\
PATH='{path}' \\\n\
PKG_CONFIG_ALLOW_CROSS='' \\\n\
PKG_CONFIG_SYSROOT_DIR='{pkg_config_sysroot_dir}' \\\n\
PKG_CONFIG_LIBDIR='{pkg_config_libdir}' \\\n\
PKG_CONFIG_PATH='{pkg_config_path}' \\\n\
PKG_CONFIG_PATH_FOR_TARGET='{pkg_config_path_for_target}' \\\n\
pkg-config \"$@\"\n"
        ),
    )?;
    #[cfg(unix)]
    fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o755))?;
    let _ = ctx;
    Ok(wrapper)
}

/// Generate the Meson native file used for build-machine resolution.
///
/// The cross file describes the target/host machine that the produced binaries
/// will run on. This native file is the matching build-machine view that Meson
/// uses when upstream marks programs or dependencies with `native: true`.
pub fn meson_native_file(ctx: &Context, paths: &PackagePaths) -> Result<PathBuf> {
    let native_file = paths.root.join("native.ini");
    let build_pkg_config = meson_native_pkg_config(ctx, paths)?;
    let vala = meson_vala_wrapper(ctx, paths)?;
    fs::write(
        &native_file,
        format!(
            "[binaries]\n\
# Meson uses this file for build-machine tools and dependencies requested
# with `native: true`.\n\
pkg-config = '{pkg_config}'\n\
vala = '{vala}'\n",
            pkg_config = build_pkg_config.display(),
            vala = vala.display()
        ),
    )?;
    Ok(native_file)
}
