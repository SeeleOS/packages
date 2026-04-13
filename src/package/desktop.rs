use std::fs;
use std::os::unix::fs::symlink;

use crate::build::build_autotools_with;
use crate::command::{CommandSpec, capture, make, run};
use crate::configure::configure_autotools;
use crate::cross::{TARGET_TRIPLE, pkg_env, target_env};
use crate::fs_utils::{copy_file, ensure_dir};
use crate::install::{install_autotools, install_file};
use crate::layout::{LIB_BINARY_DIR, relative_dir};
use crate::make_meson_packages;
use crate::make_meta_package;
use crate::make_package;
use crate::misc::sysroot_dir;
use crate::r#trait::apply_pkg_specific_patches;
use crate::make_autotools_packages;

use crate::package::xorg::{
    Freetype2, GuiPackage, LibSm, LibX11, LibXext, LibXfixes, LibXinerama, LibXrandr, LibXrender,
    XorgProto, Zlib,
};

fn rewrite_openbox_script(sysroot: &std::path::Path, rel: &str) -> crate::types::Result<()> {
    let path = sysroot.join(rel.trim_start_matches('/'));
    if !path.is_file() {
        return Ok(());
    }

    let content = fs::read_to_string(&path)?;
    let content = content
        .replace("#!/bin/sh", "#!/programs/bash")
        .replace("\n    sh ", "\n    /programs/bash ")
        .replace("\n    sh\t", "\n    /programs/bash\t")
        .replace("\n    sh$", "\n    /programs/bash$");
    fs::write(path, content)?;
    Ok(())
}

fn openbox_install_hook(ctx: &crate::types::Context) -> crate::types::Result<()> {
    let sysroot = sysroot_dir(ctx)?;
    for rel in [
        "/programs/openbox-session",
        "/programs/openbox-gnome-session",
        "/programs/openbox-kde-session",
        "/libexec/openbox-autostart",
    ] {
        rewrite_openbox_script(&sysroot, rel)?;
    }
    Ok(())
}

make_package!(
    Expat,
    "expat",
    tarball_url =
        "https://github.com/libexpat/libexpat/releases/download/R_2_7_3/expat-2.7.3.tar.xz",
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            configure_autotools(
                self,
                ctx,
                Vec::new(),
                vec![
                    "--without-docbook".to_string(),
                    "--without-examples".to_string(),
                    "--without-tests".to_string(),
                    "--without-xmlwf".to_string(),
                ],
                Vec::new(),
            )
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            build_autotools_with(self, ctx, Vec::new(), Vec::new())?;

            let paths = self.calc_paths(ctx);
            let lib_dir = paths.src.join("lib");
            let so_name = "libexpat.so.1.11.1";
            run(target_env(
                CommandSpec::new("clang")
                    .cwd(&lib_dir)
                    .arg("--target=x86_64-seele")
                    .arg("-shared")
                    .arg("-Wl,-soname,libexpat.so.1")
                    .arg("-o")
                    .arg(lib_dir.join(".libs").join(so_name))
                    .arg("xmlparse.o")
                    .arg("xmltok.o")
                    .arg("xmlrole.o")
                    .arg("-lm"),
                ctx,
            )?)
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            install_autotools(self, ctx)?;

            let paths = self.calc_paths(ctx);
            let sysroot = sysroot_dir(ctx)?;
            let target_lib_dir = sysroot.join(LIB_BINARY_DIR.trim_start_matches('/'));
            let built_lib_dir = paths.src.join("lib").join(".libs");
            let so_name = "libexpat.so.1.11.1";
            let target_so = target_lib_dir.join(so_name);
            let target_soname = target_lib_dir.join("libexpat.so.1");
            let target_link = target_lib_dir.join("libexpat.so");

            ensure_dir(&target_lib_dir)?;
            copy_file(&built_lib_dir.join(so_name), &target_so)?;

            for link in [&target_soname, &target_link] {
                if link.exists() || link.is_symlink() {
                    fs::remove_file(link)?;
                }
            }
            symlink(so_name, &target_soname)?;
            symlink(so_name, &target_link)?;
            Ok(())
        }
    }
);

make_package!(
    LiberationFonts,
    "liberation-fonts",
    tarball_url = "https://github.com/liberationfonts/liberation-fonts/files/7261482/liberation-fonts-ttf-2.1.5.tar.gz",
    package_impl = {
        fn configure(&self, _ctx: &crate::types::Context) -> crate::types::Result<()> {
            Ok(())
        }

        fn build(&self, _ctx: &crate::types::Context) -> crate::types::Result<()> {
            Ok(())
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            let sysroot = sysroot_dir(ctx)?;
            let font_dir = sysroot.join(relative_dir("/share/fonts/truetype/liberation"));

            ensure_dir(&font_dir)?;
            for entry in fs::read_dir(&paths.src)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "ttf") {
                    copy_file(&path, &font_dir.join(entry.file_name()))?;
                }
            }

            Ok(())
        }
    }
);

make_package!(
    Fontconfig,
    "fontconfig",
    tarball_url = "https://gitlab.freedesktop.org/api/v4/projects/890/packages/generic/fontconfig/2.17.1/fontconfig-2.17.1.tar.xz",
    dependencies = [Expat, Freetype2],
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            configure_autotools(
                self,
                ctx,
                Vec::new(),
                vec![
                    "--disable-docs".to_string(),
                    "--with-expat-includes=/home/elysia/coding-project/elysia-os/packages/work/sysroot-stage/libs/include".to_string(),
                    "--with-expat-lib=/home/elysia/coding-project/elysia-os/packages/work/sysroot-stage/libs/lib_binaries".to_string(),
                ],
                Vec::new(),
            )
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            build_autotools_with(self, ctx, Vec::new(), Vec::new())
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            install_autotools(self, ctx)?;

            let sysroot = sysroot_dir(ctx)?;
            let fonts_dir = sysroot.join(relative_dir("/etc/fonts"));
            ensure_dir(&fonts_dir)?;
            fs::write(
                fonts_dir.join("fonts.conf"),
                r#"<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "urn:fontconfig:fonts.dtd">
<fontconfig>
  <dir>/share/fonts</dir>
  <dir>/share/fonts/truetype</dir>
  <dir>/share/fonts/truetype/liberation</dir>

  <cachedir>/tmp/fontconfig</cachedir>

  <match target="pattern">
    <test name="family" qual="any">
      <string>monospace</string>
    </test>
    <edit name="family" mode="prepend" binding="strong">
      <string>Liberation Mono</string>
    </edit>
  </match>

  <alias>
    <family>monospace</family>
    <prefer>
      <family>Liberation Mono</family>
    </prefer>
  </alias>
</fontconfig>
"#,
            )?;

            Ok(())
        }
    }
);

make_autotools_packages!(
    { Gettext, "gettext", tarball_url = "https://ftp.gnu.org/pub/gnu/gettext/gettext-0.26.tar.gz", dependencies = [LibIconv], configure = { args = vec!["--disable-java".to_string(), "--disable-csharp".to_string(), "--disable-openmp".to_string(), "--disable-native-java".to_string(), "--without-emacs".to_string(), "--without-git".to_string(), "--without-cvs".to_string(), "--without-xz".to_string(), "--without-bzip2".to_string()] } },
    { LibFfi, "libffi", tarball_url = "https://github.com/libffi/libffi/releases/download/v3.4.8/libffi-3.4.8.tar.gz", configure = { args = vec!["--disable-docs".to_string()] } },
    { LibIconv, "libiconv", tarball_url = "https://ftp.gnu.org/pub/gnu/libiconv/libiconv-1.19.tar.gz" },
    { Pcre2, "pcre2", tarball_url = "https://github.com/PCRE2Project/pcre2/releases/download/pcre2-10.46/pcre2-10.46.tar.bz2", configure = { args = vec!["--enable-pcre2-8".to_string(), "--disable-pcre2-16".to_string(), "--disable-pcre2-32".to_string(), "--disable-jit".to_string(), "--disable-pcre2grep-jit".to_string(), "--disable-pcre2grep-callout".to_string(), "--disable-pcre2grep-callout-fork".to_string()] } },
    { Fribidi, "fribidi", tarball_url = "https://github.com/fribidi/fribidi/releases/download/v1.0.16/fribidi-1.0.16.tar.xz" },
    { LibXft, "libxft", tarball_url = "https://www.x.org/archive/individual/lib/libXft-2.3.9.tar.xz", dependencies = [XorgProto, LibX11, LibXrender, Freetype2, Fontconfig] },
    { LibXcursor, "libxcursor", tarball_url = "https://www.x.org/archive/individual/lib/libXcursor-1.2.3.tar.xz", dependencies = [XorgProto, LibX11, LibXfixes, LibXrender] },
    { LibXml2, "libxml2", tarball_url = "https://download.gnome.org/sources/libxml2/2.14/libxml2-2.14.6.tar.xz", dependencies = [Zlib], configure = { args = vec!["--without-python".to_string(), "--without-lzma".to_string(), "--without-iconv".to_string()] } },
);

make_package!(
    Openbox,
    "openbox",
    tarball_url = "https://openbox.org/dist/openbox/openbox-3.6.1.tar.gz",
    dependencies = [
        Glib2,
        Pango,
        LibXml2,
        LibXft,
        LibXcursor,
        LibXinerama,
        LibXrandr,
        LibSm,
        LibXext,
        LibX11
    ],
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            configure_autotools(
                self,
                ctx,
                Vec::new(),
                vec![
                    "--disable-imlib2".to_string(),
                    "--disable-startup-notification".to_string(),
                    "--disable-nls".to_string(),
                ],
                Vec::new(),
            )
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            build_autotools_with(self, ctx, Vec::new(), Vec::new())
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            install_autotools(self, ctx)?;
            openbox_install_hook(ctx)
        }
    }
);

make_package!(
    Dwm,
    "dwm",
    tarball_url = "https://dl.suckless.org/dwm/dwm-6.8.tar.gz",
    dependencies = [GuiPackage, LibXft, LibXinerama, Fontconfig, Freetype2, Gettext],
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            apply_pkg_specific_patches(&paths)?;
            copy_file(&paths.src.join("config.def.h"), &paths.src.join("config.h"))
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            let incs = capture(pkg_env(
                CommandSpec::new("pkg-config")
                    .arg("--cflags")
                    .arg("x11")
                    .arg("xinerama")
                    .arg("xft")
                    .arg("fontconfig")
                    .arg("freetype2"),
                ctx,
            )?)?;
            let libs = capture(pkg_env(
                CommandSpec::new("pkg-config")
                    .arg("--libs")
                    .arg("--static")
                    .arg("x11")
                    .arg("xinerama")
                    .arg("xft"),
                ctx,
            )?)?;
            run(target_env(
                make()
                    .cwd(&paths.src)
                    .arg(format!("CC=clang --target={TARGET_TRIPLE}"))
                    .arg(format!("INCS={}", incs.trim()))
                    .arg(format!("LIBS={} -lintl -liconv", libs.trim())),
                ctx,
            )?)
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            let sysroot = sysroot_dir(ctx)?;
            let man_dir = sysroot.join(relative_dir("/share/man/man1"));

            install_file(self, &paths.src.join("dwm"), &ctx.install_dir.join("dwm"))?;

            ensure_dir(&man_dir)?;
            fs::write(
                man_dir.join("dwm.1"),
                fs::read_to_string(paths.src.join("dwm.1"))?.replace("VERSION", "6.8"),
            )?;
            Ok(())
        }
    }
);

make_meson_packages!(
    { Glib2, "glib2", tarball_url = "https://download.gnome.org/sources/glib/2.84/glib-2.84.4.tar.xz", dependencies = [Gettext, LibFfi, LibIconv, Pcre2], configure = { args = vec!["-Dtests=false".to_string(), "-Dinstalled_tests=false".to_string(), "-Dintrospection=disabled".to_string(), "-Dnls=disabled".to_string(), "-Dxattr=false".to_string(), "-Dselinux=disabled".to_string(), "-Dlibmount=disabled".to_string(), "-Ddtrace=disabled".to_string(), "-Dsystemtap=disabled".to_string(), "-Dsysprof=disabled".to_string(), "-Dlibelf=disabled".to_string()] } },
    { Harfbuzz, "harfbuzz", tarball_url = "https://github.com/harfbuzz/harfbuzz/releases/download/11.4.4/harfbuzz-11.4.4.tar.xz", dependencies = [Glib2, Freetype2], configure = { args = vec!["-Dtests=disabled".to_string(), "-Ddocs=disabled".to_string(), "-Dbenchmark=disabled".to_string()] } },
    { Pango, "pango", tarball_url = "https://download.gnome.org/sources/pango/1.56/pango-1.56.4.tar.xz", dependencies = [Glib2, Harfbuzz, Fribidi, Fontconfig, Freetype2, LibXft], configure = { args = vec!["-Dbuild-testsuite=false".to_string(), "-Dbuild-examples=false".to_string(), "-Dintrospection=disabled".to_string(), "-Dgtk_doc=false".to_string(), "-Dcairo=disabled".to_string()] } },
);

pub struct OpenboxStackPackage;
make_meta_package!(
    "openbox-stack",
    OpenboxStackPackage,
    Glib2,
    LibXml2,
    Pango,
    Openbox
);
