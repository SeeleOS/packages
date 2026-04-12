use std::fs;
use std::path::Path;

use crate::misc::walk_files;
use crate::types::Result;

fn set_shell_var(script: &mut String, key: &str, value: &str) {
    let needle = format!("{key}=");
    let mut found = false;
    let mut lines = Vec::new();
    for line in script.lines() {
        if line.starts_with(&needle) {
            lines.push(format!("{needle}{value}"));
            found = true;
        } else {
            lines.push(line.to_string());
        }
    }
    if !found {
        lines.push(format!("{needle}{value}"));
    }
    *script = lines.join("\n");
    if !script.ends_with('\n') {
        script.push('\n');
    }
}

pub fn fix_libtool_scripts(root: &Path) -> Result<()> {
    let mut files = Vec::new();
    walk_files(root, &mut files)?;
    for file in files {
        if file.file_name().and_then(|name| name.to_str()) != Some("libtool") {
            continue;
        }
        let mut content = fs::read_to_string(&file)?;
        let original = content.clone();
        set_shell_var(&mut content, "host_os", "linux-gnu");
        set_shell_var(&mut content, "build_libtool_libs", "yes");
        set_shell_var(&mut content, "build_old_libs", "no");
        set_shell_var(&mut content, "version_type", "linux");
        set_shell_var(&mut content, "need_lib_prefix", "no");
        set_shell_var(&mut content, "need_version", "no");
        set_shell_var(&mut content, "deplibs_check_method", "\"pass_all\"");
        set_shell_var(&mut content, "link_all_deplibs", "yes");
        set_shell_var(
            &mut content,
            "library_names_spec",
            "'${libname}${release}${shared_ext}$versuffix ${libname}${release}${shared_ext}$major ${libname}${shared_ext}'",
        );
        set_shell_var(
            &mut content,
            "soname_spec",
            "'${libname}${release}${shared_ext}$major'",
        );
        set_shell_var(&mut content, "dynamic_linker", "'Seele ld.so'");
        set_shell_var(&mut content, "shlibpath_var", "LD_LIBRARY_PATH");
        if content != original {
            fs::write(&file, content)?;
        }
    }
    Ok(())
}
