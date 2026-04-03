use std::fs;
use std::path::Path;

use crate::misc::walk_files;
use crate::types::{Context, Result};

fn patch_config_sub_for_seele(path: &Path) -> Result<()> {
    let mut content = fs::read_to_string(path)?;
    let original = content.clone();
    if !content.contains("| seele-* \\") {
        content = content.replace("| ironclad-* \\", "| ironclad-* \\\n\t\t\t| seele-* \\");
    }
    if !content.contains("\tseele*)") {
        content = content.replace(
            "\tironclad*)\n\t\tkernel=ironclad\n\t\tos=`echo \"$basic_os\" | sed -e 's|ironclad|gnu|'`\n\t\t;;",
            "\tironclad*)\n\t\tkernel=ironclad\n\t\tos=`echo \"$basic_os\" | sed -e 's|ironclad|gnu|'`\n\t\t;;\n\tseele*)\n\t\tkernel=seele\n\t\tos=`echo \"$basic_os\" | sed -e 's|seele|gnu|'`\n\t\t;;",
        );
    }
    if !content.contains("\tseele-gnu*-)") {
        content = content.replace(
            "\tironclad-gnu*-)\n\t\t;;",
            "\tironclad-gnu*-)\n\t\t;;\n\tseele-gnu*-)\n\t\t;;",
        );
    }
    if content != original {
        fs::write(path, content)?;
    }
    Ok(())
}

pub fn refresh_gnu_config(ctx: &Context, source_dir: &Path) -> Result<()> {
    let sub = ctx.packages_root.join("config.sub");
    let guess = ctx.packages_root.join("config.guess");
    let mut files = Vec::new();
    walk_files(source_dir, &mut files)?;
    for file in files {
        match file.file_name().and_then(|name| name.to_str()) {
            Some("config.sub") => {
                fs::copy(&sub, &file)?;
                patch_config_sub_for_seele(&file)?;
            }
            Some("config.guess") => {
                fs::copy(&guess, &file)?;
            }
            _ => {}
        }
    }
    Ok(())
}
