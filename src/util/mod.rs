pub mod error;
pub mod files;
/// Provides various utility functions
pub mod macros;

use std::error::Error;
use std::{fs, path::PathBuf};

use anyhow::Context;

pub const MISSING_CFG: &str = "Failed to find the project's config file (project.lua)";

pub const DEFAULT_COMPILER: &str = "gcc";

#[inline(always)]
pub fn root_dir_name(cur_dir: &PathBuf) -> Option<&str> {
    cur_dir.file_name()?.to_str()
}

#[inline(always)]
pub fn create_dir(dir: &str) -> anyhow::Result<()> {
    fs::create_dir(dir).context(format!("Failed to create directory: {}", dir))
}

// recursively go through directory
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

#[inline(always)]
pub fn get_header_files(path: &PathBuf) -> Vec<PathBuf> {
    get_files(path, ".h")
}

#[inline(always)]
pub fn get_src_files(path: &PathBuf) -> Vec<PathBuf> {
    get_files(path, ".c")
}

pub fn result_to_option<T, E: Error>(res: Result<T, E>) -> Option<T> {
    match res {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}
