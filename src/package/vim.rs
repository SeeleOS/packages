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
            vec![
                ("CC".to_string(), "clang --target=x86_64-seele".to_string()),
                ("LIBS".to_string(), "-lncurses -ltinfo".to_string()),
            ],
            vec![
                "--with-tlib=ncurses".to_string(),
                "--with-features=normal".to_string(),
                "--enable-multibyte".to_string(),
                "--disable-gui".to_string(),
                "--without-x".to_string(),
                "--disable-acl".to_string(),
                "--disable-gpm".to_string(),
                "--disable-sysmouse".to_string(),
                "--disable-nls".to_string(),
                "--disable-netbeans".to_string(),
                "--enable-channel=no".to_string(),
                "--enable-terminal=no".to_string(),
                "--enable-perlinterp=no".to_string(),
                "--enable-pythoninterp=no".to_string(),
                "--enable-python3interp=no".to_string(),
                "--enable-rubyinterp=no".to_string(),
                "--enable-luainterp=no".to_string(),
                "--enable-mzschemeinterp=no".to_string(),
                "--enable-tclinterp=no".to_string(),
            ],
            Vec::new(),
        )
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        build_make_in(
            &self.calc_paths(ctx).src,
            vec![("VIMRUNTIMEDIR".to_string(), "/misc/vim".to_string())],
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
