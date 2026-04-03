use crate::make_autotools_package;
use crate::package::xorg::{XorgProto, Xtrans};

make_autotools_package!(
    LibIce,
    "libice",
    tarball_url = "https://www.x.org/archive/individual/lib/libICE-1.1.2.tar.gz",
    dependencies = [XorgProto, Xtrans],
);
