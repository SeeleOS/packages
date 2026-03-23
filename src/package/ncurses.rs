use crate::build::CC;
use crate::command::{CommandSpec, capture, run};
use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::fs_utils::{copy_file_with_sudo, ensure_dir, verify_same_size};
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct Ncurses;

const BUILD_CC: &str = "gcc";

impl Package for Ncurses {
    fn name(&self) -> &'static str {
        "ncurses"
    }

    fetch_wrap!(TarballFetch);

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);

        let build_triplet = capture(CommandSpec::new("../config.guess").cwd(&paths.src))?;

        run(CommandSpec::new("../configure")
            .cwd(&paths.build)
            .env("CC", CC)
            .arg(format!("--build={}", build_triplet.trim()))
            .arg("--host=x86_64-unknown-none")
            .arg("--prefix=/")
            .arg("--bindir=/programs")
            .arg("--includedir=/misc/libs/system_include")
            .arg("--libdir=/misc/libs/system_lib")
            .arg("--without-ada")
            .arg("--without-cxx")
            .arg("--without-cxx-binding")
            .arg("--without-manpages")
            .arg("--without-tests")
            .arg("--without-progs")
            .arg("--with-normal")
            .arg("--without-shared")
            .arg("--with-termlib")
            .arg("--disable-db-install")
            .arg("--with-fallbacks=xterm,vt100,linux")
            .arg("--disable-stripping")
            .arg("--disable-widec")
            .arg(format!("--with-build-cc={BUILD_CC}")))?;
        Ok(())
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        run(CommandSpec::new("make").cwd(&self.calc_paths(ctx).build))?;
        Ok(())
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        run(CommandSpec::new("sudo")
            .arg("mkdir")
            .arg("-p")
            .arg(&ctx.system_include_dir))?;
        run(CommandSpec::new("sudo")
            .arg("mkdir")
            .arg("-p")
            .arg(&ctx.system_lib_dir))?;

        let headers = [
            "curses.h",
            "eti.h",
            "form.h",
            "menu.h",
            "ncurses_cfg.h",
            "ncurses_def.h",
            "ncurses_dll.h",
            "panel.h",
            "parametrized.h",
            "term.h",
            "termcap.h",
            "unctrl.h",
        ];

        let libs = [
            "libform.a",
            "libmenu.a",
            "libncurses.a",
            "libpanel.a",
            "libtinfo.a",
        ];

        for header in headers {
            let source = paths.build.join("include").join(header);
            let target = ctx.system_include_dir.join(header);
            copy_file_with_sudo(&source, &target)?;
            verify_same_size(&source, &target)?;
        }

        for lib in libs {
            let source = paths.build.join("lib").join(lib);
            let target = ctx.system_lib_dir.join(lib);
            copy_file_with_sudo(&source, &target)?;
            verify_same_size(&source, &target)?;
        }

        Ok(())
    }
}

impl TarballFetch for Ncurses {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://ftp.gnu.org/gnu/ncurses/ncurses-6.6.tar.gz"]
    }
}
