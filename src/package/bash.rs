use crate::build::CC;
use crate::command::{CommandSpec, make, run};
use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::fs_utils::{copy_file_with_sudo, ensure_dir, verify_same_size};
use crate::r#trait::Package;
use crate::types::{Context, Result};

pub struct Bash;

const CC_FOR_BUILD: &str = "gcc";
const CFLAGS_FOR_BUILD: &str = "-g -DCROSS_COMPILING -std=gnu17";
const ADDON_LDFLAGS: &str = "-Wl,--allow-multiple-definition";
const BASH_CV_GETENV_REDEF: &str = "no";
const BASH_CV_GETCWD_MALLOC: &str = "yes";
const BASH_CV_FUNC_STRCHRNUL_WORKS: &str = "yes";

impl Package for Bash {
    fn name(&self) -> &'static str {
        "bash"
    }

    fetch_wrap!(TarballFetch);

    fn configure(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        ensure_dir(&paths.build)?;

        run(CommandSpec::new("../configure")
            .cwd(&paths.build)
            .env("CC", CC)
            .env("CC_FOR_BUILD", CC_FOR_BUILD)
            .env("CFLAGS_FOR_BUILD", CFLAGS_FOR_BUILD)
            .env("bash_cv_getenv_redef", BASH_CV_GETENV_REDEF)
            .env("bash_cv_getcwd_malloc", BASH_CV_GETCWD_MALLOC)
            .env("bash_cv_func_strchrnul_works", BASH_CV_FUNC_STRCHRNUL_WORKS)
            .arg("--host=x86_64-unknown-none")
            .arg("--prefix=/")
            .arg("--disable-nls")
            .arg("--without-bash-malloc"))?;
        Ok(())
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        run(make()
            .cwd(&self.calc_paths(ctx).build)
            .env("CC_FOR_BUILD", CC_FOR_BUILD)
            .env("CFLAGS_FOR_BUILD", CFLAGS_FOR_BUILD)
            .env("ADDON_LDFLAGS", ADDON_LDFLAGS)
            .arg("bash"))?;
        Ok(())
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let source = paths.build.join("bash");
        let target = ctx.install_dir.join("bash");
        copy_file_with_sudo(&source, &target)?;
        verify_same_size(&source, &target)?;
        Ok(())
    }
}

impl TarballFetch for Bash {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://ftp.gnu.org/gnu/bash/bash-5.3.tar.gz"]
    }
}
