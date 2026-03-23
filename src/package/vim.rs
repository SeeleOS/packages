use crate::command::{CommandSpec, run};
use crate::fetch::GitCloneFetch;
use crate::fetch_wrap;
use crate::fs_utils::{copy_file_with_sudo, verify_same_size};
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct Vim;

impl Package for Vim {
    fn name(&self) -> &'static str {
        "vim"
    }

    fetch_wrap!(GitCloneFetch);

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);

        run(CommandSpec::new("./configure")
            .cwd(&paths.src)
            .env("CC", "clang --target=x86_64-seele")
            .env("LIBS", "-lncurses -ltinfo")
            .arg("--build=x86_64-pc-linux-gnu")
            .arg("--host=x86_64-seele")
            .arg("--target=x86_64-seele")
            .arg("--prefix=/")
            .arg("--with-tlib=ncurses")
            .arg("--with-features=normal")
            .arg("--enable-multibyte")
            .arg("--disable-gui")
            .arg("--without-x")
            .arg("--disable-acl")
            .arg("--disable-gpm")
            .arg("--disable-sysmouse")
            .arg("--disable-nls")
            .arg("--disable-netbeans")
            .arg("--enable-channel=no")
            .arg("--enable-terminal=no")
            .arg("--enable-perlinterp=no")
            .arg("--enable-pythoninterp=no")
            .arg("--enable-python3interp=no")
            .arg("--enable-rubyinterp=no")
            .arg("--enable-luainterp=no")
            .arg("--enable-mzschemeinterp=no")
            .arg("--enable-tclinterp=no"))?;
        Ok(())
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        run(CommandSpec::new("make").cwd(&self.calc_paths(ctx).src))?;
        Ok(())
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let source = paths.src.join("src/vim");
        let target = ctx.install_dir.join("vim");
        copy_file_with_sudo(&source, &target)?;
        verify_same_size(&source, &target)?;
        Ok(())
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
