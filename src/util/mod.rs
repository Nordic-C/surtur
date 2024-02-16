pub mod error;
/// Provides various utility functions
pub mod macros;

use std::{fs, path::PathBuf};

pub const MISSING_CFG: &str = "Failed to find the project's config file (project.lua)";

pub fn root_dir_name(cur_dir: &str) -> &str {
    let dirs: Vec<&str> = cur_dir.split('/').collect();
    dirs.last().unwrap_or_else(|| {
        panic!(
            "Failed to get current dir. Provided dir: {} is invalid",
            cur_dir
        )
    })
}

pub fn create_dir(dir: &str) {
    match fs::create_dir(dir) {
        Ok(_) => (),
        Err(err) => panic!("Failed to create dir: {}", err),
    }
}

pub fn traverse_path(path: &PathBuf, files: &mut Vec<PathBuf>) {
    let dir = fs::read_dir(path)
        .unwrap_or_else(|_| panic!("Failed to find directory: {}", path.display()));
    for entry in dir.flatten() {
        let file_type = entry.file_type().expect("Failed to get file type");
        if file_type.is_dir() {
            traverse_path(&entry.path(), files);
        } else {
            files.push(entry.path())
        }
    }
}
