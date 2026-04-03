use crate::fs_utils::copy_file_with_sudo;
use crate::misc::sysroot_dir;
use crate::types::{Context, Result};

pub fn xorg_server_install_hook(ctx: &Context) -> Result<()> {
    let source = ctx.packages_root.join("xorg-server/xorg.conf");
    let target = sysroot_dir(ctx)?.join("etc/X11/xorg.conf");
    copy_file_with_sudo(&source, &target)
}
