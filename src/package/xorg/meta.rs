use crate::make_meta_package;

use super::{XorgServer, XorgTwm, XorgXeyes, XorgXinit};

pub struct GuiPackage;
make_meta_package!("gui", GuiPackage, XorgServer, XorgXinit, XorgTwm, XorgXeyes);

pub struct XorgPackage;
make_meta_package!("xorg", XorgPackage, GuiPackage);
