use crate::build::build_make_in;
use crate::command::{CommandSpec, run};
use crate::configure::configure_autotools;
use crate::install::{prune_libtool_archives, with_clean_install_env};
use crate::make_autotools_packages;
use crate::make_meson_packages;
use crate::make_package;
use crate::misc::sysroot_dir;
use crate::package::desktop::{Fontconfig, Glib2, LibXcursor, LibXml2, Pango};
use crate::package::feh::LibPng;
use crate::package::xorg::{
    Freetype2, LibX11, LibXext, LibXi, LibXinerama, LibXrandr, LibXrender, Pixman, Zlib,
};

make_meson_packages!(
    { Atk, "atk", tarball_url = "https://download.gnome.org/sources/atk/2.38/atk-2.38.0.tar.xz", dependencies = [Glib2], configure = { args = vec!["-Dintrospection=false".to_string(), "-Ddocs=false".to_string()] } },
    { AppStream, "appstream", tarball_url = "https://github.com/ximion/appstream/archive/refs/tags/v1.1.2.tar.gz", dependencies = [Glib2, Curl, LibFyaml, LibXml2, LibXmlb], configure = { args = vec!["-Dapidocs=false".to_string(), "-Dbash-completion=false".to_string(), "-Dcompose=false".to_string(), "-Ddocs=false".to_string(), "-Dgir=false".to_string(), "-Dinstall-docs=false".to_string(), "-Dman=false".to_string(), "-Dstemming=false".to_string(), "-Dsvg-support=false".to_string(), "-Dsystemd=false".to_string(), "-Dzstd-support=false".to_string()] } },
    { Cairo, "cairo", tarball_url = "https://www.cairographics.org/releases/cairo-1.18.4.tar.xz", dependencies = [Glib2, Pixman, Zlib, LibPng, Freetype2, Fontconfig, LibX11, LibXext, LibXrender], configure = { args = vec!["-Dtests=disabled".to_string(), "-Dgtk_doc=false".to_string(), "-Dxcb=disabled".to_string(), "-Dxlib=enabled".to_string(), "-Dfontconfig=enabled".to_string(), "-Dfreetype=enabled".to_string(), "-Dpng=enabled".to_string(), "-Dglib=enabled".to_string()] } },
    { LibEpoxy, "libepoxy", tarball_url = "https://github.com/anholt/libepoxy/archive/refs/tags/1.5.10.tar.gz", dependencies = [LibX11], configure = { args = vec!["-Dtests=false".to_string(), "-Degl=no".to_string(), "-Dglx=yes".to_string(), "-Dx11=true".to_string()] } },
    { Graphene, "graphene", tarball_url = "https://download.gnome.org/sources/graphene/1.10/graphene-1.10.8.tar.xz", dependencies = [Glib2], configure = { env = vec![("CPPFLAGS".to_string(), "-D__BSD_VISIBLE=0 -D__XSI_VISIBLE=0".to_string()), ("CFLAGS".to_string(), "-Wno-error=undef -Wno-nan-infinity-disabled".to_string())], args = vec!["-Dtests=false".to_string(), "-Dintrospection=disabled".to_string()] } },
    { GdkPixbuf, "gdk-pixbuf", tarball_url = "https://download.gnome.org/sources/gdk-pixbuf/2.42/gdk-pixbuf-2.42.12.tar.xz", dependencies = [Glib2, LibPng], configure = { args = vec!["-Dintrospection=disabled".to_string(), "-Dinstalled_tests=false".to_string(), "-Dman=false".to_string(), "-Dgio_sniffing=false".to_string(), "-Dpng=enabled".to_string(), "-Djpeg=disabled".to_string(), "-Dtiff=disabled".to_string()] } },
    { LibGnomeGamesSupport, "libgnome-games-support", tarball_url = "https://download.gnome.org/sources/libgnome-games-support/2.0/libgnome-games-support-2.0.2.tar.xz", dependencies = [Glib2, Gtk4, LibAdwaita], configure = { args = vec!["-Dtests=false".to_string()] } },
    { LibAdwaita, "libadwaita", tarball_url = "https://download.gnome.org/sources/libadwaita/1.7/libadwaita-1.7.4.tar.xz", dependencies = [AppStream, Glib2, Gtk4], configure = { args = vec!["-Dintrospection=disabled".to_string(), "-Ddocumentation=false".to_string(), "-Dexamples=false".to_string(), "-Dtests=false".to_string()] } },
    { Librsvg, "librsvg", tarball_url = "https://download.gnome.org/sources/librsvg/2.60/librsvg-2.60.2.tar.xz", dependencies = [Cairo, GdkPixbuf, Glib2, LibXml2, Pango], configure = { args = vec!["-Dpixbuf-loader=disabled".to_string(), "-Dtests=false".to_string(), "-Dtools=false".to_string(), "-Dvala=disabled".to_string()] } },
    { LibXmlb, "libxmlb", tarball_url = "https://github.com/hughsie/libxmlb/releases/download/0.3.25/libxmlb-0.3.25.tar.xz", dependencies = [Glib2], configure = { args = vec!["-Dcli=false".to_string(), "-Dgtkdoc=false".to_string(), "-Dintrospection=false".to_string(), "-Dlzma=disabled".to_string(), "-Dtests=false".to_string(), "-Dzstd=disabled".to_string()] } },
    { GnomeMines, "gnome-mines", tarball_url = "https://download.gnome.org/sources/gnome-mines/48/gnome-mines-48.1.tar.xz", dependencies = [Glib2, Gtk4, LibGee, LibAdwaita, Librsvg, LibGnomeGamesSupport], configure = { args = vec![] } },
    { Gtk3, "gtk3", tarball_url = "https://download.gnome.org/sources/gtk+/3.24/gtk+-3.24.43.tar.xz", dependencies = [Atk, Cairo, GdkPixbuf, Glib2, Pango, LibEpoxy, LibX11, LibXcursor, LibXext, LibXi, LibXinerama, LibXrandr, LibXrender], configure = { args = vec!["-Dintrospection=false".to_string(), "-Ddemos=false".to_string(), "-Dexamples=false".to_string(), "-Dtests=false".to_string(), "-Dwayland_backend=false".to_string(), "-Dx11_backend=true".to_string(), "-Dbroadway_backend=false".to_string()] } },
    { Gtk4, "gtk4", tarball_url = "https://download.gnome.org/sources/gtk/4.18/gtk-4.18.6.tar.xz", dependencies = [Cairo, GdkPixbuf, Glib2, Pango, LibEpoxy, Graphene, LibX11, LibXcursor, LibXi, LibXrandr, LibXrender], configure = { args = vec!["-Dintrospection=disabled".to_string(), "-Dbuild-demos=false".to_string(), "-Dbuild-examples=false".to_string(), "-Dbuild-tests=false".to_string(), "-Dbuild-testsuite=false".to_string(), "-Dwayland-backend=false".to_string(), "-Dx11-backend=true".to_string(), "-Dbroadway-backend=false".to_string(), "-Dmedia-gstreamer=disabled".to_string(), "-Dprint-cups=disabled".to_string(), "-Dvulkan=disabled".to_string()] } },
);

make_package!(
    Curl,
    "curl",
    tarball_url = "https://curl.se/download/curl-8.18.0.tar.xz",
    dependencies = [Zlib],
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            configure_autotools(
                self,
                ctx,
                Vec::new(),
                vec![
                    "--enable-shared".to_string(),
                    "--disable-static".to_string(),
                    "--disable-docs".to_string(),
                    "--disable-manual".to_string(),
                    "--disable-threaded-resolver".to_string(),
                    "--without-brotli".to_string(),
                    "--without-ca-bundle".to_string(),
                    "--without-ca-path".to_string(),
                    "--without-libidn2".to_string(),
                    "--without-libpsl".to_string(),
                    "--without-librtmp".to_string(),
                    "--without-nghttp2".to_string(),
                    "--without-nghttp3".to_string(),
                    "--without-ngtcp2".to_string(),
                    "--without-ssl".to_string(),
                    "--without-zstd".to_string(),
                    "--disable-ares".to_string(),
                    "--disable-ldap".to_string(),
                    "--disable-ldaps".to_string(),
                    "--disable-rtsp".to_string(),
                    "--disable-dict".to_string(),
                    "--disable-telnet".to_string(),
                    "--disable-tftp".to_string(),
                    "--disable-pop3".to_string(),
                    "--disable-imap".to_string(),
                    "--disable-smb".to_string(),
                    "--disable-smtp".to_string(),
                    "--disable-gopher".to_string(),
                    "--disable-mqtt".to_string(),
                    "--disable-ipfs".to_string(),
                    "--disable-websockets".to_string(),
                ],
                Vec::new(),
            )
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            build_make_in(&paths.src.join("include"), Vec::new(), Vec::new())?;
            build_make_in(&paths.src.join("lib"), Vec::new(), Vec::new())
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            let sysroot = sysroot_dir(ctx)?;

            run(with_clean_install_env(
                CommandSpec::new("env")
                    .arg(format!("DESTDIR={}", sysroot.display()))
                    .arg("make")
                    .arg("-C")
                    .arg(paths.src.join("include"))
                    .arg("install"),
            ))?;
            run(with_clean_install_env(
                CommandSpec::new("env")
                    .arg(format!("DESTDIR={}", sysroot.display()))
                    .arg("make")
                    .arg("-C")
                    .arg(paths.src.join("lib"))
                    .arg("install"),
            ))?;
            run(with_clean_install_env(
                CommandSpec::new("env")
                    .arg(format!("DESTDIR={}", sysroot.display()))
                    .arg("make")
                    .arg("-C")
                    .arg(&paths.src)
                    .arg("install-pkgconfigDATA"),
            ))?;

            prune_libtool_archives(&sysroot)
        }
    }
);

make_autotools_packages!(
    { LibGee, "libgee", tarball_url = "https://download.gnome.org/sources/libgee/0.20/libgee-0.20.8.tar.xz", dependencies = [Glib2], configure = { args = vec!["--disable-introspection".to_string()] } },
    { LibFyaml, "libfyaml", tarball_url = "https://github.com/pantoniou/libfyaml/releases/download/v0.9.4/libfyaml-0.9.4.tar.gz", configure = { args = vec!["--disable-network".to_string()] } },
    { Gtk2, "gtk2", tarball_url = "https://download.gnome.org/sources/gtk+/2.24/gtk+-2.24.33.tar.xz", dependencies = [Atk, Cairo, GdkPixbuf, Glib2, Pango, LibX11, LibXcursor, LibXext, LibXi, LibXinerama, LibXrandr, LibXrender], configure = { args = vec!["--disable-glibtest".to_string(), "--disable-cups".to_string(), "--disable-modules".to_string(), "--disable-papi".to_string(), "--disable-test-print-backends".to_string()] } },
);
