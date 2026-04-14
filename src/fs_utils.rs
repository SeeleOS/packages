use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use crate::command::{CommandSpec, run};
use crate::types::Result;

pub fn ensure_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn remove_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn remove_path_if_exists(path: &Path) -> Result<()> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    if metadata.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn touch(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    fs::write(path, [])?;
    Ok(())
}

pub fn list_patch_files(dir: &Path) -> Result<Vec<PathBuf>> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if matches!(
            path.extension(),
            Some(ext) if ext == OsStr::new("patch") || ext == OsStr::new("diff")
        ) {
            entries.push(path);
        }
    }
    Ok(entries)
}

pub fn copy_file(from: &Path, to: &Path) -> Result<()> {
    let parent = to.parent().ok_or("install target has no parent")?;
    ensure_dir(parent)?;
    remove_path_if_exists(to)?;
    fs::copy(from, to)?;
    Ok(())
}

pub fn copy_dir_contents(from: &Path, to: &Path) -> Result<()> {
    ensure_dir(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let source = entry.path();
        let target = to.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_contents(&source, &target)?;
        } else if file_type.is_symlink() {
            let link_target = fs::read_link(&source)?;
            create_symlink_force(&link_target, &target)?;
        } else {
            copy_file(&source, &target)?;
        }
    }
    Ok(())
}

pub fn verify_same_size(from: &Path, to: &Path) -> Result<()> {
    let local = fs::metadata(from)?.len();
    let installed = fs::metadata(to)?.len();
    if local != installed {
        return Err(format!("size mismatch for {}", to.display()).into());
    }
    Ok(())
}

pub fn create_symlink_force(target: &Path, link: &Path) -> Result<()> {
    let parent = link.parent().ok_or("symlink target has no parent")?;
    ensure_dir(parent)?;
    remove_path_if_exists(link)?;
    symlink(target, link)?;
    Ok(())
}

pub fn download_file(target: &Path, urls: &[&str], cwd: &Path) -> Result<()> {
    let downloader = if which("curl") {
        "curl"
    } else if which("wget") {
        "wget"
    } else {
        return Err("neither curl nor wget found; cannot download tarball".into());
    };

    for url in urls {
        let result = if downloader == "curl" {
            run(CommandSpec::new("curl")
                .arg("-L")
                .arg("-f")
                .arg("--retry")
                .arg("10")
                .arg("--retry-all-errors")
                .arg("--retry-delay")
                .arg("2")
                .arg("-o")
                .arg(target)
                .arg(url)
                .cwd(cwd))
        } else {
            run(CommandSpec::new("wget")
                .arg("-O")
                .arg(target)
                .arg(url)
                .cwd(cwd))
        };
        if result.is_ok() {
            return Ok(());
        }
    }
    Err(format!(
        "failed to download {} from configured URLs",
        target.display()
    )
    .into())
}

pub fn which(program: &str) -> bool {
    std::env::var_os("PATH")
        .is_some_and(|paths| std::env::split_paths(&paths).any(|dir| dir.join(program).exists()))
}
