use crate::{
    command::{CommandSpec, capture, run},
    fs_utils::{ensure_dir, touch},
    types::{Context, PackagePaths, Result},
};
use std::fs;
use std::path::{Path, PathBuf};

pub fn stamp(name: &str, paths: &PackagePaths) -> Result<()> {
    ensure_dir(&paths.stamp)?;
    touch(&paths.stamp.join(name))?;
    Ok(())
}

pub fn with_stamp<F>(
    func: F,
    name: &str,
    paths: &PackagePaths,
    rebuild: bool,
    ignore_rebuild: bool,
) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    let stamp_path = paths.stamp.join(name);
    let should_run = !stamp_path.exists() || (rebuild && !ignore_rebuild);

    if should_run {
        func()?;
        stamp(name, paths)?;
    }

    Ok(())
}

pub fn mount_sysroot() -> Result<()> {
    let project_root = discover_project_root()?;
    let sysroot = project_root.join("sysroot");
    let disk_img = project_root.join("disk.img");

    ensure_dir(&sysroot)?;

    if capture(CommandSpec::new("mountpoint").arg("-q").arg(&sysroot)).is_ok() {
        return Ok(());
    }

    run(CommandSpec::new("sudo")
        .arg("mount")
        .arg("-o")
        .arg("loop")
        .arg(&disk_img)
        .arg(&sysroot))
}

pub fn umount_sysroot() -> Result<()> {
    let project_root = discover_project_root()?;
    let sysroot = project_root.join("sysroot");

    if capture(CommandSpec::new("mountpoint").arg("-q").arg(&sysroot)).is_err() {
        return Ok(());
    }

    run(CommandSpec::new("sudo").arg("umount").arg(&sysroot))
}

pub fn sysroot_dir(ctx: &Context) -> Result<PathBuf> {
    Ok(ctx.staging_sysroot_dir.clone())
}

pub fn deployed_sysroot_dir(ctx: &Context) -> Result<PathBuf> {
    Ok(ctx.real_sysroot_dir.clone())
}

pub fn walk_files(root: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            walk_files(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

fn discover_project_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;

    for dir in cwd.ancestors() {
        if dir.join("packages").join("README.md").is_file() && dir.join("disk.img").is_file() {
            return Ok(dir.to_path_buf());
        }
        if dir.join("README.md").is_file()
            && dir.join("src").is_dir()
            && dir.file_name().is_some_and(|name| name == "packages")
        {
            return dir
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(|| "packages directory has no parent".into());
        }
    }

    Err("could not locate project root from current working directory".into())
}
