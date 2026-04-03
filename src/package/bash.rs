use crate::build::{CC, build_make_in};
use crate::configure::configure_autotools;
use crate::fetch::TarballFetch;
use crate::fetch_wrap;
use crate::install::{Install, install_file};
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
        configure_autotools(
            self,
            ctx,
            vec![
                ("CC".to_string(), CC.to_string()),
                ("CC_FOR_BUILD".to_string(), CC_FOR_BUILD.to_string()),
                ("CFLAGS_FOR_BUILD".to_string(), CFLAGS_FOR_BUILD.to_string()),
                ("bash_cv_getenv_redef".to_string(), BASH_CV_GETENV_REDEF.to_string()),
                ("bash_cv_getcwd_malloc".to_string(), BASH_CV_GETCWD_MALLOC.to_string()),
                ("bash_cv_func_strchrnul_works".to_string(), BASH_CV_FUNC_STRCHRNUL_WORKS.to_string()),
            ],
            vec!["--disable-nls".to_string(), "--without-bash-malloc".to_string()],
            Vec::new(),
        )
    }

    fn build(&self, ctx: &Context) -> Result<()> {
        build_make_in(
            &self.calc_paths(ctx).src,
            vec![
                ("CC_FOR_BUILD".to_string(), CC_FOR_BUILD.to_string()),
                ("CFLAGS_FOR_BUILD".to_string(), CFLAGS_FOR_BUILD.to_string()),
                ("ADDON_LDFLAGS".to_string(), ADDON_LDFLAGS.to_string()),
            ],
            vec!["bash".to_string()],
        )
    }

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let source = paths.src.join("bash");
        let target = ctx.install_dir.join("bash");
        install_file(self, &source, &target)
    }
}

impl Install for Bash {
    fn binary_name(&self) -> &'static str {
        "bash"
    }
}

impl TarballFetch for Bash {
    fn tarball_url(&self) -> Vec<&'static str> {
        vec!["https://ftp.gnu.org/gnu/bash/bash-5.3.tar.gz"]
    }
}
