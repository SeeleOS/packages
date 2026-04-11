use std::fs;

use crate::command::{CommandSpec, make, run};
use crate::cross::{TARGET_TRIPLE, target_env};
use crate::fs_utils::{copy_file, ensure_dir};
use crate::install::install_file;
use crate::layout::relative_dir;
use crate::make_package;
use crate::misc::sysroot_dir;
use crate::package::desktop::{Fontconfig, LiberationFonts, LibXft};
use crate::package::xorg::{Freetype2, LibX11};

const VERSION: &str = "0.9.3";

make_package!(
    St,
    "st",
    tarball_url = "https://dl.suckless.org/st/st-0.9.3.tar.gz",
    dependencies = [LibX11, LibXft, Fontconfig, Freetype2, LiberationFonts],
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            copy_file(&paths.src.join("config.def.h"), &paths.src.join("config.h"))
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            run(
                target_env(
                    make()
                        .cwd(&paths.src)
                        .arg(format!("CC=clang --target={TARGET_TRIPLE}"))
                        .arg(format!("X11INC={}", ctx.include_root_dir.display()))
                        .arg(format!("X11LIB={}", ctx.lib_binary_dir.display())),
                    ctx,
                )?,
            )
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            let sysroot = sysroot_dir(ctx)?;
            let man_dir = sysroot.join(relative_dir("/share/man/man1"));
            let terminfo_dir = sysroot.join(relative_dir("/share/terminfo"));

            install_file(self, &paths.src.join("st"), &ctx.install_dir.join("st"))?;

            ensure_dir(&man_dir)?;
            fs::write(
                man_dir.join("st.1"),
                fs::read_to_string(paths.src.join("st.1"))?.replace("VERSION", VERSION),
            )?;

            ensure_dir(&terminfo_dir)?;
            run(
                CommandSpec::new("tic")
                    .cwd(&paths.src)
                    .arg("-sx")
                    .arg("-o")
                    .arg(&terminfo_dir)
                    .arg("st.info"),
            )
        }
    }
);
