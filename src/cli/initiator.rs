/// Handling of initialization and
/// fixing of projects. This will
/// add missing config files or
/// fix issues [WIP]
/// 
/// Later on, it will also be able
/// to migrate Make projects

use std::fs;

use super::creator::Project;

pub fn init_proj(proj: &Project) -> anyhow::Result<()> {
    let cfg_file = proj.root_dir.join("project.lua");
    let main_file = proj.root_dir.join("src").join("main.c");
    if fs::metadata(cfg_file).is_err() {
        Project::create_cfg_file(proj.root_dir, &proj.name, false)?;
    }

    if fs::metadata(main_file).is_err() {
        Project::create_main_file(proj.root_dir, false)?;
    }

    Ok(())
}
