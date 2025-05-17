pub mod files;
pub mod lua_utils;
/// Provides various utility functions
pub mod macros;

use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::{exit, Command};
use std::{error::Error, fs, path::PathBuf};

use anyhow::Context;
use colored::Colorize;

pub const MISSING_CFG: &str = "Failed to find the project's config file (project.lua)";

pub const DEFAULT_COMPILER: &str = "gcc";

#[inline(always)]
pub fn root_dir_name(cur_dir: &Path) -> Option<&str> {
    cur_dir.file_name()?.to_str()
}

#[inline(always)]
pub fn create_dir(dir: &str) -> anyhow::Result<()> {
    fs::create_dir(dir).context(format!("Failed to create directory: {}", dir))
}

pub fn run_c_program(cmd: &mut Command, cur_dir: &PathBuf) -> anyhow::Result<()> {
    env::set_var("SURTUR_PROJ_DIR", cur_dir);

    match cmd.status() {
        Ok(status) => {
            let code = status.code();
            match code {
                Some(c) => exit(c),
                None => {
                    println!("{}", "Program exited by throwing an error".red());
                    println!("{}: ... xDDDD did you seriously think C would show you the error? Pathetic.", "Error".red());
                    exit(1);
                }
            }
        }
        Err(err) => {
            Err(err).context("Failed to run the c program. Execution of the program failed.")
        }
    }
}

// recursively go through directory
// TODO: Remove recursion as it creates a bunch of unnessecary heap allocations
fn get_files(path: &PathBuf, ending: &str) -> HashSet<PathBuf> {
    let dir = fs::read_dir(path)
        .unwrap_or_else(|_| panic!("Failed to find directory: {}", path.display()));
    dir.flatten()
        .flat_map(|entry| {
            let file_type = entry.file_type().expect("Failed to get file type");
            if file_type.is_dir() {
                get_files(&entry.path(), ending)
            } else {
                let file_name = entry.file_name().to_string_lossy().to_string();
                let file_ending = &file_name[file_name.len() - 2..];
                if file_ending == ending {
                    HashSet::from([entry.path()])
                } else {
                    HashSet::new()
                }
            }
        })
        .collect()
}

#[inline(always)]
pub fn get_header_files(path: &PathBuf) -> HashSet<PathBuf> {
    get_files(path, ".h")
}

#[inline(always)]
pub fn get_src_files(path: &PathBuf) -> HashSet<PathBuf> {
    get_files(path, ".c")
}

pub fn result_to_option<T, E: Error>(res: Result<T, E>) -> Option<T> {
    match res {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}
