/// Provides various utility functions

pub mod macros;
pub mod error;

use std::fs;

pub const MISSING_CFG: &str = "Failed to find the project's config file (project.lua)";

pub fn root_dir_name(cur_dir: &str) -> &str {
    let dirs: Vec<&str> = cur_dir.split('/').collect();
    dirs.last().unwrap_or_else(|| panic!("Failed to get current dir. Provided dir: {} is invalid", cur_dir))
}

pub fn create_dir(dir: &str) {
    match fs::create_dir(dir) {
        Ok(_) => (),
        Err(err) => panic!("Failed to create dir: {}", err),
    }
}