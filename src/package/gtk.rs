use crate::make_autotools_packages;
use crate::make_meson_packages;
use crate::package::desktop::{Fontconfig, Glib2, LibXcursor, Pango};
use crate::package::feh::LibPng;
use crate::package::xorg::{
    Freetype2, LibX11, LibXext, LibXi, LibXinerama, LibXrandr, LibXrender, Pixman, Zlib,
};

make_meson_packages!(
    { Atk, "atk", tarball_url = "https://download.gnome.org/sources/atk/2.38/atk-2.38.0.tar.xz", dependencies = [Glib2], configure = { args = vec!["-Dintrospection=disabled".to_string(), "-Dtests=false".to_string()] } },
    { Cairo, "cairo", tarball_url = "https://www.cairographics.org/releases/cairo-1.18.4.tar.xz", dependencies = [Pixman, Zlib, LibPng, Freetype2, Fontconfig, LibX11, LibXext, LibXrender], configure = { args = vec!["-Dtests=disabled".to_string(), "-Dgtk_doc=disabled".to_string(), "-Dxcb=disabled".to_string(), "-Dxlib=enabled".to_string(), "-Dxlib-xrender=enabled".to_string(), "-Dfontconfig=enabled".to_string(), "-Dfreetype=enabled".to_string(), "-Dpng=enabled".to_string(), "-Dglib=disabled".to_string()] } },
    { GdkPixbuf, "gdk-pixbuf", tarball_url = "https://download.gnome.org/sources/gdk-pixbuf/2.42/gdk-pixbuf-2.42.12.tar.xz", dependencies = [Glib2, LibPng], configure = { args = vec!["-Dintrospection=disabled".to_string(), "-Dinstalled_tests=false".to_string(), "-Dman=false".to_string(), "-Dpng=enabled".to_string(), "-Djpeg=disabled".to_string(), "-Dtiff=disabled".to_string()] } },
    { Gtk3, "gtk3", tarball_url = "https://download.gnome.org/sources/gtk+/3.24/gtk+-3.24.43.tar.xz", dependencies = [Atk, Cairo, GdkPixbuf, Glib2, Pango, LibX11, LibXcursor, LibXext, LibXi, LibXinerama, LibXrandr, LibXrender], configure = { args = vec!["-Dintrospection=disabled".to_string(), "-Ddemos=false".to_string(), "-Dexamples=false".to_string(), "-Dtests=false".to_string(), "-Dwayland_backend=false".to_string(), "-Dx11_backend=true".to_string(), "-Dbroadway_backend=false".to_string()] } },
    { Gtk4, "gtk4", tarball_url = "https://download.gnome.org/sources/gtk/4.18/gtk-4.18.6.tar.xz", dependencies = [Cairo, GdkPixbuf, Glib2, Pango, LibX11, LibXcursor, LibXi, LibXrandr, LibXrender], configure = { args = vec!["-Dintrospection=disabled".to_string(), "-Dbuild-demos=false".to_string(), "-Dbuild-examples=false".to_string(), "-Dbuild-tests=false".to_string(), "-Dwayland-backend=false".to_string(), "-Dx11-backend=true".to_string(), "-Dbroadway-backend=false".to_string(), "-Dmedia-gstreamer=disabled".to_string(), "-Dprint-cups=disabled".to_string()] } },
);

make_autotools_packages!(
    { Gtk2, "gtk2", tarball_url = "https://download.gnome.org/sources/gtk+/2.24/gtk+-2.24.33.tar.xz", dependencies = [Atk, Cairo, GdkPixbuf, Glib2, Pango, LibX11, LibXcursor, LibXext, LibXi, LibXinerama, LibXrandr, LibXrender], configure = { args = vec!["--disable-glibtest".to_string(), "--disable-cups".to_string(), "--disable-modules".to_string(), "--disable-papi".to_string(), "--disable-test-print-backends".to_string()] } },
);
