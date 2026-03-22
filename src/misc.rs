use crate::{
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
