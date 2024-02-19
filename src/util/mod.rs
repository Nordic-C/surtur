pub mod error;
/// Provides various utility functions
pub mod macros;

use std::{fs, path::PathBuf};

pub const MISSING_CFG: &str = "Failed to find the project's config file (project.lua)";

pub const DEFAULT_COMPILER: &str = "gcc";

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

pub fn get_files(path: &PathBuf, ending: &str) -> Vec<PathBuf> {
    let dir = fs::read_dir(path)
        .unwrap_or_else(|_| panic!("Failed to find directory: {}", path.display()));
    dir.flatten()
        .map(|entry| {
            let file_type = entry.file_type().expect("Failed to get file type");
            if file_type.is_dir() {
                get_files(&entry.path(), ending)
            } else {
                let file_name = entry.file_name().to_string_lossy().to_string();
                let file_ending = &file_name[file_name.len() - 2..];
                if file_ending == ending {
                    vec![entry.path()]
                } else {
                    vec![]
                }
            }
        })
        .flatten()
        .collect()
}

pub fn get_header_files(path: &PathBuf) -> Vec<PathBuf> {
    get_files(path, ".h")
}

pub fn get_src_files(path: &PathBuf) -> Vec<PathBuf> {
    get_files(path, ".c")
}
