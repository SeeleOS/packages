use std::fs;
use std::path::{Path, PathBuf};

use crate::command::{CommandSpec, capture};
use crate::misc::sysroot_dir;
use crate::types::{Context, PackagePaths, Result};

pub const TARGET_TRIPLE: &str = "x86_64-seele";

const TARGET_CC: &str = "clang";
const TARGET_CXX: &str = "clang++";
const TARGET_AR: &str = "llvm-ar";
const TARGET_NM: &str = "llvm-nm";
const TARGET_STRIP: &str = "llvm-strip";

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
    let llvm_bin = ctx
        .packages_root
        .parent()
        .ok_or("packages directory has no parent")?
        .join(".llvm/bin");
    let current_path = std::env::var_os("PATH").unwrap_or_default();
    let path = if current_path.is_empty() {
        llvm_bin.into_os_string()
    } else {
        let mut merged = llvm_bin.into_os_string();
        merged.push(":");
        merged.push(current_path);
        merged
    };
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
        .env("PATH", path)
        .env("PKG_CONFIG_ALLOW_CROSS", "1")
        .env("PKG_CONFIG_SYSROOT_DIR", sysroot_dir(ctx)?)
        .env("PKG_CONFIG_LIBDIR", &pkg_config_path)
        .env("PKG_CONFIG_PATH", &pkg_config_path)
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
                "-L{} -Wl,-rpath-link,{}",
                ctx.lib_binary_dir.display(),
                ctx.lib_binary_dir.display()
            ),
            " ",
        ))
}

pub fn meson_cross_file(ctx: &Context, paths: &PackagePaths) -> Result<PathBuf> {
    let cross_file = paths.root.join("seele.cross");
    fs::write(
        &cross_file,
        format!(
            "[binaries]\n\
             c = ['{TARGET_CC}', '--target={TARGET_TRIPLE}']\n\
             cpp = ['{TARGET_CXX}', '--target={TARGET_TRIPLE}']\n\
             ar = '{TARGET_AR}'\n\
             nm = '{TARGET_NM}'\n\
             strip = '{TARGET_STRIP}'\n\
             pkg-config = 'pkg-config'\n\
             \n[built-in options]\n\
             c_args = ['-fPIC', '-I{root_inc}', '-I{c_inc}']\n\
             cpp_args = ['-fPIC', '-idirafter{root_inc}', '-idirafter{c_inc}']\n\
             c_link_args = ['-L{lib}', '-Wl,-rpath-link,{lib}']\n\
             cpp_link_args = ['-L{lib}', '-Wl,-rpath-link,{lib}']\n\
             \n[properties]\nneeds_exe_wrapper = true\n\
             \n[host_machine]\n\
             system = 'seele'\ncpu_family = 'x86_64'\ncpu = 'x86_64'\nendian = 'little'\n",
            root_inc = ctx.include_root_dir.display(),
            c_inc = ctx.include_c_dir.display(),
            lib = ctx.lib_binary_dir.display(),
        ),
    )?;
    Ok(cross_file)
}
