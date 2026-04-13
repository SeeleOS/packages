use std::fs;

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
    dependencies = [LibPng, Freetype2, LibX11, LibXext, LibXcb, LibSm, Gettext]
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
