use std::path::{Path, PathBuf};

use crate::{
    build::build_cmake,
    command::{CommandSpec, run},
    configure::with_cmake_layout,
    cross::{TARGET_TRIPLE, pkg_env},
    fs_utils::ensure_dir,
    install::install_cmake,
    misc::sysroot_dir,
    r#trait::Package,
    types::{Context, Result},
};

const LLVM_TARGET_TRIPLE: &str = "x86_64-unknown-seele";

pub struct Clang;

impl Package for Clang {
    fn name(&self) -> &'static str {
        "clang"
    }

    fn fetch(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let source_root = llvm_source_root(ctx)?;
        ensure_dir(&paths.root)?;
        ensure_dir(&paths.src)?;

        run(CommandSpec::new("rsync")
            .arg("-a")
            .arg("--delete")
            .arg("--exclude=.git")
            .arg("--exclude=build")
            .arg("--exclude=build-*")
            .arg(format!("{}/", source_root.display()))
            .arg(format!("{}/", paths.src.display())))?;

        Ok(())
    }

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let sysroot = sysroot_dir(ctx)?;
        let llvm_prefix = llvm_prefix(ctx)?;
        let llvm_bin = llvm_prefix.join("bin");
        let target_lib_dir = llvm_prefix.join("lib").join(LLVM_TARGET_TRIPLE);
        let target_cpp_include = llvm_prefix
            .join("include")
            .join(LLVM_TARGET_TRIPLE)
            .join("c++")
            .join("v1");
        let generic_cpp_include = llvm_prefix.join("include").join("c++").join("v1");

        ensure_dir(&paths.build)?;

        let c_flags = format!(
            "-fPIC -I{} -I{}",
            ctx.include_root_dir.display(),
            ctx.include_c_dir.display()
        );
        let cxx_flags = format!(
            "-fPIC -nostdinc++ -I{} -I{} -I{} -I{}",
            generic_cpp_include.display(),
            target_cpp_include.display(),
            ctx.include_root_dir.display(),
            ctx.include_c_dir.display()
        );
        let link_flags = format!(
            "-L{} -Wl,-rpath-link,{} -L{} -Wl,-rpath-link,{}",
            ctx.lib_binary_dir.display(),
            ctx.lib_binary_dir.display(),
            target_lib_dir.display(),
            target_lib_dir.display()
        );

        let mut cmd = with_cmake_layout(pkg_env(CommandSpec::new("cmake").cwd(&paths.root), ctx)?)
            .arg("-S")
            .arg(paths.src.join("llvm"))
            .arg("-B")
            .arg(&paths.build)
            .arg("-G")
            .arg("Ninja")
            .arg("-DCMAKE_BUILD_TYPE=Release")
            .arg("-DCMAKE_SYSTEM_NAME=Seele")
            .arg("-DCMAKE_C_COMPILER=clang")
            .arg("-DCMAKE_CXX_COMPILER=clang++")
            .arg(format!("-DCMAKE_C_COMPILER_TARGET={TARGET_TRIPLE}"))
            .arg(format!("-DCMAKE_CXX_COMPILER_TARGET={TARGET_TRIPLE}"))
            .arg(format!("-DCMAKE_ASM_COMPILER_TARGET={TARGET_TRIPLE}"))
            .arg(format!("-DCMAKE_SYSROOT={}", sysroot.display()))
            .arg("-DCMAKE_TRY_COMPILE_TARGET_TYPE=STATIC_LIBRARY")
            .arg(format!("-DCMAKE_C_FLAGS={c_flags}"))
            .arg(format!("-DCMAKE_CXX_FLAGS={cxx_flags}"))
            .arg(format!("-DCMAKE_EXE_LINKER_FLAGS={link_flags}"))
            .arg(format!("-DCMAKE_SHARED_LINKER_FLAGS={link_flags}"))
            .arg("-DLLVM_ENABLE_PROJECTS=clang")
            .arg("-DLLVM_TARGETS_TO_BUILD=X86")
            .arg("-DLLVM_BUILD_TOOLS=ON")
            .arg("-DLLVM_INCLUDE_TESTS=OFF")
            .arg("-DLLVM_INCLUDE_EXAMPLES=OFF")
            .arg("-DLLVM_INCLUDE_BENCHMARKS=OFF")
            .arg("-DLLVM_INCLUDE_UTILS=OFF")
            .arg("-DLLVM_ENABLE_BACKTRACES=OFF")
            .arg("-DLLVM_ENABLE_BINDINGS=OFF")
            .arg("-DLLVM_ENABLE_LIBEDIT=OFF")
            .arg("-DLLVM_ENABLE_LIBXML2=OFF")
            .arg("-DLLVM_ENABLE_ZLIB=OFF")
            .arg("-DLLVM_ENABLE_ZSTD=OFF")
            .arg("-DCLANG_BUILD_TOOLS=ON")
            .arg("-DCLANG_ENABLE_STATIC_ANALYZER=OFF")
            .arg("-DCLANG_ENABLE_OBJC_REWRITER=OFF")
            .arg(format!(
                "-DLLVM_DEFAULT_TARGET_TRIPLE={LLVM_TARGET_TRIPLE}"
            ))
            .arg("-DCLANG_DEFAULT_CXX_STDLIB=libc++")
            .arg("-DCLANG_DEFAULT_RTLIB=compiler-rt")
            .arg("-DCLANG_DEFAULT_UNWINDLIB=libunwind")
            .arg("-DDEFAULT_SYSROOT=/")
            .arg(format!("-DLLVM_NATIVE_TOOL_DIR={}", llvm_bin.display()))
            .arg(format!(
                "-DLLVM_TABLEGEN={}",
                llvm_bin.join("llvm-tblgen").display()
            ))
            .arg(format!(
                "-DCLANG_TABLEGEN={}",
                llvm_bin.join("clang-tblgen").display()
            ));

        if target_lib_dir.is_dir() {
            cmd = cmd.arg(format!("-DCMAKE_PREFIX_PATH={}", target_lib_dir.display()));
        }

        run(cmd)
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        build_cmake(self, ctx)
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        install_cmake(self, ctx)
    }
}

fn project_root(ctx: &Context) -> Result<&Path> {
    ctx.packages_root
        .parent()
        .ok_or_else(|| "packages directory has no parent".into())
}

fn llvm_source_root(ctx: &Context) -> Result<PathBuf> {
    let source = project_root(ctx)?.join("toolchain").join("llvm-project");
    if !source.join("llvm").join("CMakeLists.txt").is_file() {
        return Err(format!(
            "local llvm-project source tree not found at {}",
            source.display()
        )
        .into());
    }
    Ok(source)
}

fn llvm_prefix(ctx: &Context) -> Result<PathBuf> {
    let prefix = project_root(ctx)?.join(".llvm");
    if !prefix.join("bin").join("clang").exists() {
        return Err(format!(
            "host LLVM toolchain not found at {}; run toolchain/install.rs first",
            prefix.display()
        )
        .into());
    }
    Ok(prefix)
}
