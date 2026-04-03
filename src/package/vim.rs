use crate::build::build_make_in;
use crate::configure::configure_autotools;
use crate::fetch::GitCloneFetch;
use crate::fetch_wrap;
use crate::install::{install_dir_contents, install_file};
use crate::package::ncurses::Ncurses;
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct Vim;

impl Package for Vim {
    fn name(&self) -> &'static str {
        "vim"
    }

    fn dependencies(&self) -> Vec<Box<dyn Package>> {
        vec![Box::new(Ncurses)]
    }

    fetch_wrap!(GitCloneFetch);

    fn configure(&self, ctx: &Context) -> Result<()> {
        configure_autotools(
            self,
            ctx,
            &[("CC", "clang --target=x86_64-seele"), ("LIBS", "-lncurses -ltinfo")],
            &[
                "--with-tlib=ncurses",
                "--with-features=normal",
                "--enable-multibyte",
                "--disable-gui",
                "--without-x",
                "--disable-acl",
                "--disable-gpm",
                "--disable-sysmouse",
                "--disable-nls",
                "--disable-netbeans",
                "--enable-channel=no",
                "--enable-terminal=no",
                "--enable-perlinterp=no",
                "--enable-pythoninterp=no",
                "--enable-python3interp=no",
                "--enable-rubyinterp=no",
                "--enable-luainterp=no",
                "--enable-mzschemeinterp=no",
                "--enable-tclinterp=no",
            ],
            Vec::new(),
        )
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        build_make_in(
            &self.calc_paths(ctx).src,
            &[("VIMRUNTIMEDIR", "/misc/vim")],
            Vec::new(),
        )
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let source = paths.src.join("src/vim");
        let target = ctx.install_dir.join("vim");
        let sysroot = ctx
            .install_dir
            .parent()
            .ok_or("install_dir has no parent")?;
        let runtime_source = paths.src.join("runtime");
        let runtime_target = sysroot.join("misc/vim");

        install_file(self, &source, &target)?;
        install_dir_contents(self, &runtime_source, &runtime_target)
    }
}

impl GitCloneFetch for Vim {
    fn git_url(&self) -> &'static str {
        "https://github.com/vim/vim.git"
    }

    fn git_commit(&self) -> &'static str {
        "0172ff5"
    }
}
