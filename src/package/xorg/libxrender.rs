use crate::make_autotools_package;
use crate::package::xorg::{LibX11, XorgProto};

make_autotools_package!(
    LibXrender,
    "libxrender",
    tarball_url = "https://www.x.org/archive/individual/lib/libXrender-0.9.12.tar.gz",
    dependencies = [XorgProto, LibX11],
);
