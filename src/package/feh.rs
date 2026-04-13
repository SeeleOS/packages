use std::fs;
use std::os::unix::fs::symlink;

use crate::fs_utils::{copy_file, ensure_dir};
use crate::layout::{LIB_BINARY_DIR, relative_dir};
use crate::make_autotools_package;
use crate::package::desktop::Gettext;
use crate::package::xorg::{
    Freetype2, LibSm, LibX11, LibXcb, LibXext, LibXinerama, LibXt, XorgProto, Zlib,
};

make_autotools_package!(
    LibPng,
    "libpng",
    tarball_url = "https://download.sourceforge.net/libpng/libpng-1.6.55.tar.xz",
    dependencies = [Zlib],
    configure = {
        args = vec![
            "--disable-tests".to_string(),
            "--disable-tools".to_string(),
        ]
    }
);

make_autotools_package!(
    Imlib2,
    "imlib2",
    tarball_url = "https://download.sourceforge.net/enlightenment/imlib2-1.12.6.tar.xz",
    dependencies = [LibPng, Freetype2, LibX11, LibXext, LibXcb, LibSm, Gettext],
    configure = {
        args = vec![
            "--without-x-shm-fd".to_string(),
            "--disable-static".to_string(),
            "--disable-fast-install".to_string(),
            "--without-bzip2".to_string(),
            "--without-lzma".to_string(),
            "--without-zlib".to_string(),
            "--without-jpeg".to_string(),
            "--without-tiff".to_string(),
            "--without-gif".to_string(),
            "--without-webp".to_string(),
            "--without-heif".to_string(),
            "--without-jxl".to_string(),
            "--without-avif".to_string(),
            "--without-openjpeg".to_string(),
            "--without-librsvg".to_string(),
            "--without-libid3tag".to_string(),
            "--without-raw".to_string(),
            "--without-libspectre".to_string(),
            "--without-libjxl".to_string(),
            "--without-libwebp".to_string(),
            "--without-libheif".to_string(),
            "--without-libavif".to_string(),
            "--without-libopenjp2".to_string(),
            "--without-libraw".to_string(),
            "--without-libspectre".to_string(),
        ]
    },
    install_override = {
        let cwd = std::env::current_dir()?;
        let packages_root = if cwd.join("README.md").is_file() && cwd.join("pkg-specific").is_dir()
        {
            cwd
        } else if cwd.join("packages").join("README.md").is_file() {
            cwd.join("packages")
        } else {
            return Err("could not locate packages directory from current working directory".into());
        };
        let paths = packages_root.join("work/imlib2/src");
        let sysroot = packages_root.join("work/sysroot-stage");
        let lib_dir = sysroot.join(relative_dir(LIB_BINARY_DIR));
        let include_dir = sysroot.join("libs/include");
        let pc_dir = lib_dir.join("pkgconfig");
        let loaders_dir = lib_dir.join("imlib2/loaders");
        let filters_dir = lib_dir.join("imlib2/filters");
        let built_lib_dir = paths.join("src/lib/.libs");

        ensure_dir(&lib_dir)?;
        ensure_dir(&include_dir)?;
        ensure_dir(&pc_dir)?;
        ensure_dir(&loaders_dir)?;
        ensure_dir(&filters_dir)?;

        copy_file(&built_lib_dir.join("libImlib2.so.1.12.6"), &lib_dir.join("libImlib2.so.1.12.6"))?;
        for link in ["libImlib2.so.1", "libImlib2.so"] {
            let link_path = lib_dir.join(link);
            if link_path.exists() || link_path.is_symlink() {
                fs::remove_file(&link_path)?;
            }
            symlink("libImlib2.so.1.12.6", &link_path)?;
        }

        copy_file(&paths.join("src/lib/Imlib2.h"), &include_dir.join("Imlib2.h"))?;
        copy_file(
            &paths.join("src/lib/Imlib2_Loader.h"),
            &include_dir.join("Imlib2_Loader.h"),
        )?;
        copy_file(&paths.join("imlib2.pc"), &pc_dir.join("imlib2.pc"))?;

        for dir in [("src/modules/loaders/.libs", &loaders_dir), ("src/modules/filters/.libs", &filters_dir)] {
            for entry in fs::read_dir(paths.join(dir.0))? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "so") {
                    copy_file(&path, &dir.1.join(entry.file_name()))?;
                }
            }
        }
    }
);

make_autotools_package!(
    Feh,
    "feh",
    tarball_url = "https://feh.finalrewind.org/feh-3.12.1.tar.bz2",
    dependencies = [Imlib2, LibPng, LibX11, LibXt, LibXinerama, XorgProto],
    build = {
        args = vec![
            "curl=0".to_string(),
            "exif=0".to_string(),
            "help=0".to_string(),
            "inotify=0".to_string(),
            "magic=0".to_string(),
            "verscmp=0".to_string(),
            "xinerama=1".to_string(),
        ]
    },
    configure_override = {
        let cwd = std::env::current_dir()?;
        let packages_root = if cwd.join("README.md").is_file() && cwd.join("pkg-specific").is_dir()
        {
            cwd
        } else if cwd.join("packages").join("README.md").is_file() {
            cwd.join("packages")
        } else {
            return Err("could not locate packages directory from current working directory".into());
        };
        let config_mk = packages_root.join("work/feh/src/config.mk");
        if config_mk.is_file() {
            let content = fs::read_to_string(&config_mk)?;
            let updated = content.replace("PREFIX ?= /usr/local", "PREFIX ?= /");
            let updated = updated.replace("PREFIX = /usr/local", "PREFIX = /");
            if updated != content {
                fs::write(config_mk, updated)?;
            }
        }
    }
);
