use crate::{
    make_meta_package,
    meta_pkg::MetaPackage,
    package::{bash::Bash, busybox::Busybox, tinycc::TinyCc},
    r#trait::Package,
};

pub struct BasePackage;

make_meta_package!("base", BasePackage, Bash, Busybox, TinyCc);
