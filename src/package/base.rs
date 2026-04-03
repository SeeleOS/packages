use crate::{
    make_meta_package,
    package::{bash::Bash, busybox::Busybox, tinycc::TinyCc},
};

pub struct BasePackage;

make_meta_package!("base", BasePackage, Bash, Busybox, TinyCc);
