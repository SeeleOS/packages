use crate::{
    command::{CommandSpec, run},
    fs_utils::{copy_dir_contents, copy_file, create_symlink_force, ensure_dir, verify_same_size},
    layout::LIB_BINARY_DIR,
    misc::{deployed_sysroot_dir, mount_sysroot, sysroot_dir, umount_sysroot, walk_files},
    r#trait::Package,
    types::{Context, Result},
};
use std::fs;

pub fn with_clean_install_env<'a>(spec: CommandSpec<'a>) -> CommandSpec<'a> {
    spec.env_remove("LD_LIBRARY_PATH")
        .env_remove("LD_PRELOAD")
        .env_remove("LD_AUDIT")
}

#[macro_export]
macro_rules! install_wrap {
    () => {
        fn install(&self, ctx: &Context) -> Result<()> {
            <Self as $crate::install::Install>::install(self, ctx)
        }
    };
}

pub trait Install: Package {
    fn binary_name(&self) -> &'static str;

    fn install(&self, ctx: &Context) -> Result<()> {
        let paths = self.calc_paths(ctx);
        let source = paths.build.join(self.binary_name());
        let target = ctx.install_dir.join(self.install_name());
        copy_file(&source, &target)?;
        verify_same_size(&source, &target)?;
        Ok(())
    }
}

pub fn install_file(
    _pkg: &dyn Package,
    source: &std::path::Path,
    target: &std::path::Path,
) -> Result<()> {
    copy_file(source, target)?;
    verify_same_size(source, target)?;
    Ok(())
}

pub fn install_dir_contents(
    _pkg: &dyn Package,
    source_dir: &std::path::Path,
    target_dir: &std::path::Path,
) -> Result<()> {
    copy_dir_contents(source_dir, target_dir)?;
    Ok(())
}

pub fn install_autotools(pkg: &dyn Package, ctx: &Context) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    install_make_in(&paths.src, ctx)
}

pub fn install_make_in(cwd: &std::path::Path, ctx: &Context) -> Result<()> {
    let sysroot = sysroot_dir(ctx)?;
    ensure_dir(&sysroot)?;
    run(with_clean_install_env(
        CommandSpec::new("env")
            .arg(format!("DESTDIR={}", sysroot.display()))
            .arg("make")
            .arg("-C")
            .arg(cwd)
            .arg("install"),
    ))?;
    postprocess_sysroot(&sysroot)
}

pub fn install_meson(pkg: &dyn Package, ctx: &Context) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    let sysroot = sysroot_dir(ctx)?;
    ensure_dir(&sysroot)?;
    run(with_clean_install_env(
        CommandSpec::new("env")
            .arg(format!("DESTDIR={}", sysroot.display()))
            .arg("meson")
            .arg("install")
            .arg("--no-rebuild")
            .arg("-C")
            .arg(&paths.build),
    ))?;
    postprocess_sysroot(&sysroot)
}

pub fn install_cmake(pkg: &dyn Package, ctx: &Context) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    let sysroot = sysroot_dir(ctx)?;
    ensure_dir(&sysroot)?;
    run(with_clean_install_env(
        CommandSpec::new("env")
            .arg(format!("DESTDIR={}", sysroot.display()))
            .arg("cmake")
            .arg("--install")
            .arg(&paths.build),
    ))?;
    postprocess_sysroot(&sysroot)
}

pub fn install_cargo(
    pkg: &dyn Package,
    ctx: &Context,
    bins: Vec<String>,
    profile: &str,
) -> Result<()> {
    let paths = pkg.calc_paths(ctx);
    let bins = if bins.is_empty() {
        vec![pkg.install_name().to_string()]
    } else {
        bins
    };

    for bin in bins {
        let source = paths
            .build
            .join("target")
            .join(crate::cross::TARGET_TRIPLE)
            .join(profile)
            .join(&bin);
        let target = ctx.install_dir.join(&bin);
        install_file(pkg, &source, &target)?;
    }

    Ok(())
}

pub fn deploy_sysroot(ctx: &Context) -> Result<()> {
    let staging = sysroot_dir(ctx)?;
    let deployed = deployed_sysroot_dir(ctx)?;

    ensure_dir(&staging)?;
    mount_sysroot()?;
    run(CommandSpec::new("sudo")
        .arg("mkdir")
        .arg("-p")
        .arg(&deployed))?;
    run(CommandSpec::new("sudo")
        .arg("rsync")
        .arg("-a")
        .arg("--keep-dirlinks")
        .arg(format!("{}/", staging.display()))
        .arg(&deployed))?;
    run(CommandSpec::new("sync"))?;
    umount_sysroot()
}

pub fn prune_libtool_archives(root: &std::path::Path) -> Result<()> {
    let mut files = Vec::new();
    walk_files(root, &mut files)?;
    for path in files {
        if path.extension().is_some_and(|ext| ext == "la") {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn postprocess_sysroot(root: &std::path::Path) -> Result<()> {
    prune_libtool_archives(root)?;
    restore_shared_library_symlinks(&root.join(LIB_BINARY_DIR.trim_start_matches('/')))
}

fn restore_shared_library_symlinks(libdir: &std::path::Path) -> Result<()> {
    if !libdir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(libdir)? {
        let entry = entry?;
        if entry.file_type()?.is_symlink() {
            continue;
        }
        let file_name = entry.file_name().to_string_lossy().into_owned();
        if !file_name.starts_with("lib") {
            continue;
        }
        let Some(soname_suffix) = file_name.strip_prefix("lib").and_then(|name| name.split_once(".so.")) else {
            continue;
        };
        let (stem, version) = soname_suffix;
        if stem.is_empty() || version.is_empty() {
            continue;
        }

        let linker_name = format!("lib{stem}.so");
        let soname_name = format!("lib{stem}.so.{}", version.split('.').next().unwrap_or(version));
        if linker_name != file_name {
            create_symlink_force(std::path::Path::new(&file_name), &libdir.join(linker_name))?;
        }
        if soname_name != file_name {
            create_symlink_force(std::path::Path::new(&file_name), &libdir.join(soname_name))?;
        }
    }

    Ok(())
}
