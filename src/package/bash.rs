use crate::build::{CC, build_make_in};
use crate::configure::configure_autotools;
use crate::install::install_file;
use crate::make_package;

const CC_FOR_BUILD: &str = "gcc";
const CFLAGS_FOR_BUILD: &str = "-g -DCROSS_COMPILING -std=gnu17";
const ADDON_LDFLAGS: &str = "-Wl,--allow-multiple-definition";
const BASH_CV_GETENV_REDEF: &str = "no";
const BASH_CV_GETCWD_MALLOC: &str = "yes";
const BASH_CV_FUNC_STRCHRNUL_WORKS: &str = "yes";

make_package!(
    Bash,
    "bash",
    tarball_url = "https://ftp.gnu.org/gnu/bash/bash-5.3.tar.gz",
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
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

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
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

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            let source = paths.src.join("bash");
            let target = ctx.install_dir.join("bash");
            install_file(self, &source, &target)
        }
    }
);
