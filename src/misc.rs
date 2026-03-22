use crate::{
    command::{CommandSpec, run},
    fs_utils::{ensure_dir, touch},
    types::{PackagePaths, Result},
};

pub fn stamp(name: &str, paths: PackagePaths) -> Result<()> {
    ensure_dir(&paths.stamp)?;
    touch(&paths.stamp.join(name))?;
    Ok(())
}
