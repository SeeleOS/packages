use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::command::{CommandSpec, capture, run};
use crate::trace::{detail, section};
use crate::types::Result;

pub fn ensure_dir(path: &Path) -> Result<()> {
    detail(format!("ensuring directory {}", path.display()));
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn remove_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        detail(format!("removing file {}", path.display()));
        fs::remove_file(path)?;
    } else {
        detail(format!("file already absent {}", path.display()));
    }
    Ok(())
}

pub fn remove_path_if_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        detail(format!("path already absent {}", path.display()));
        return Ok(());
    }
    if path.is_dir() {
        detail(format!("removing directory tree {}", path.display()));
        fs::remove_dir_all(path)?;
    } else {
        detail(format!("removing file {}", path.display()));
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn touch(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }
    detail(format!("touching {}", path.display()));
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
        if path.extension() == Some(OsStr::new("patch")) {
            entries.push(path);
        }
    }
    Ok(entries)
}

pub fn copy_file_with_sudo(from: &Path, to: &Path) -> Result<()> {
    let parent = to.parent().ok_or("install target has no parent")?;
    section(format!(
        "copying artifact with sudo: {} -> {}",
        from.display(),
        to.display()
    ));
    ensure_dir(parent)?;
    run(CommandSpec::new("sudo").arg("mkdir").arg("-p").arg(parent))?;
    run(CommandSpec::new("sudo").arg("rm").arg("-f").arg(to))?;
    run(CommandSpec::new("sudo").arg("cp").arg(from).arg(to))?;
    run(CommandSpec::new("sync"))?;
    Ok(())
}

pub fn verify_same_size(from: &Path, to: &Path) -> Result<()> {
    let local = fs::metadata(from)?.len();
    let installed = capture(CommandSpec::new("sudo").arg("stat").arg("-c%s").arg(to))?;
    let installed = installed.trim().parse::<u64>()?;
    detail(format!(
        "verifying installed size: local={} bytes installed={} bytes target={}",
        local,
        installed,
        to.display()
    ));
    if local != installed {
        return Err(format!("size mismatch for {}", to.display()).into());
    }
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

    section(format!(
        "downloading {} with {}",
        target.display(),
        downloader
    ));
    for url in urls {
        detail(format!("trying download URL {}", url));
        let result = if downloader == "curl" {
            run(CommandSpec::new("curl")
                .arg("-L")
                .arg("-f")
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
            detail(format!("download succeeded from {}", url));
            return Ok(());
        }
        detail(format!("download failed from {}, trying next URL", url));
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
