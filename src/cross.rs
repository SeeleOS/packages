use std::fs;
use std::path::{Path, PathBuf};

use crate::command::{capture, CommandSpec};
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
    let libdir = ctx.system_lib_dir.join("pkgconfig");
    Ok(spec
        .env("PKG_CONFIG_ALLOW_CROSS", "1")
        .env("PKG_CONFIG_SYSROOT_DIR", sysroot_dir(ctx)?)
        .env("PKG_CONFIG_LIBDIR", libdir.clone())
        .env("PKG_CONFIG_PATH", libdir))
}

pub fn target_env<'a>(spec: CommandSpec<'a>, ctx: &'a Context) -> Result<CommandSpec<'a>> {
    Ok(pkg_env(spec, ctx)?
        .env("CC", format!("clang --target={TARGET_TRIPLE}"))
        .env("CXX", format!("clang++ --target={TARGET_TRIPLE}"))
        .env("AR", TARGET_AR)
        .env("NM", TARGET_NM)
        .env("RANLIB", "llvm-ranlib")
        .env("STRIP", "llvm-strip")
        .env("CPPFLAGS", format!("-I{}", ctx.system_include_dir.display()))
        .env("CFLAGS", "-fPIC")
        .env("CXXFLAGS", "-fPIC")
        .env(
            "LDFLAGS",
            format!(
                "-L{} -Wl,-rpath-link,{}",
                ctx.system_lib_dir.display(),
                ctx.system_lib_dir.display()
            ),
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
             c_args = ['-fPIC', '-I{inc}']\n\
             cpp_args = ['-fPIC', '-I{inc}']\n\
             c_link_args = ['-L{lib}', '-Wl,-rpath-link,{lib}']\n\
             cpp_link_args = ['-L{lib}', '-Wl,-rpath-link,{lib}']\n\
             \n[properties]\nneeds_exe_wrapper = true\n\
             \n[host_machine]\n\
             system = 'seele'\ncpu_family = 'x86_64'\ncpu = 'x86_64'\nendian = 'little'\n",
            inc = ctx.system_include_dir.display(),
            lib = ctx.system_lib_dir.display(),
        ),
    )?;
    Ok(cross_file)
}
