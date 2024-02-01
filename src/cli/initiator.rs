/// Handling of initialization and
/// fixing of projects. This will
/// add missing config files or
/// fix issues [WIP]
/// 
/// Later on, it will also be able
/// to migrate Make projects

use std::fs;

use super::creator::Project;

pub fn init_proj(proj: &Project) {
    let cfg_file = format!("{}/project.lua", proj.root_dir);
    let main_file = format!("{}/src/main.c", proj.root_dir);
    if fs::metadata(cfg_file).is_err() {
        Project::create_cfg_file(&proj.root_dir, &proj.name);
    }

    if fs::metadata(main_file).is_err() {
        Project::create_main_file(&proj.root_dir);
    }
}
