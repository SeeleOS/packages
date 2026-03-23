use crate::{
    command::{CommandSpec, capture, run},
    fs_utils::{ensure_dir, touch},
    trace::{detail, section},
    types::{PackagePaths, Result},
};
use std::path::PathBuf;

pub fn stamp(name: &str, paths: &PackagePaths) -> Result<()> {
    detail(format!(
        "creating stamp `{}` at {}",
        name,
        paths.stamp.join(name).display()
    ));
    ensure_dir(&paths.stamp)?;
    touch(&paths.stamp.join(name))?;
    Ok(())
}

pub fn with_stamp<F>(func: F, name: &str, paths: &PackagePaths) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    if !paths.stamp.join(name).exists() {
        detail(format!(
            "stamp `{}` missing, executing guarded step in {}",
            name,
            paths.root.display()
        ));
        func()?;
        stamp(name, paths)?;
    } else {
        detail(format!(
            "stamp `{}` already present, skipping guarded step in {}",
            name,
            paths.root.display()
        ));
    }

    Ok(())
}

pub fn mount_sysroot() -> Result<()> {
    let project_root = discover_project_root()?;
    let sysroot = project_root.join("sysroot");
    let disk_img = project_root.join("disk.img");

    section(format!(
        "ensuring sysroot mount at {} from {}",
        sysroot.display(),
        disk_img.display()
    ));
    ensure_dir(&sysroot)?;

    if capture(CommandSpec::new("mountpoint").arg("-q").arg(&sysroot)).is_ok() {
        detail(format!("sysroot {} is already mounted", sysroot.display()));
        return Ok(());
    }

    detail("sysroot is not mounted, mounting loopback image");
    run(CommandSpec::new("sudo")
        .arg("mount")
        .arg("-o")
        .arg("loop")
        .arg(&disk_img)
        .arg(&sysroot))
}

fn discover_project_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    detail(format!(
        "discovering project root from cwd={}",
        cwd.display()
    ));

    for dir in cwd.ancestors() {
        if dir.join("packages").join("README.md").is_file() && dir.join("disk.img").is_file() {
            detail(format!("found project root at {}", dir.display()));
            return Ok(dir.to_path_buf());
        }
        if dir.join("README.md").is_file()
            && dir.join("src").is_dir()
            && dir.file_name().is_some_and(|name| name == "packages")
        {
            detail(format!(
                "detected packages directory {}, using parent as project root",
                dir.display()
            ));
            return dir
                .parent()
                .map(|parent| parent.to_path_buf())
                .ok_or_else(|| "packages directory has no parent".into());
        }
    }

    Err("could not locate project root from current working directory".into())
}
