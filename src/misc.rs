use crate::{
    command::{CommandSpec, run},
    fs_utils::{ensure_dir, touch},
    types::{PackagePaths, Result},
};

pub fn stamp(name: &str, paths: &PackagePaths) -> Result<()> {
    ensure_dir(&paths.stamp)?;
    touch(&paths.stamp.join(name))?;
    Ok(())
}

pub fn with_stamp<F>(func: F, name: &str, paths: &PackagePaths) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    if !paths.stamp.join(name).exists() {
        func()?;
        stamp(name, paths)?;
    }

    Ok(())
}

pub fn mount_sysroot() -> Result<()> {
    run(CommandSpec::new("sudo")
        .arg("mount")
        .arg("../../disk.img")
        .arg("../../sysroot"))
}
