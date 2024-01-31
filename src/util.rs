/// Provides various utility functions

use std::{process, fs};

use colored::Colorize;

pub fn root_dir_name(cur_dir: &str) -> &str {
    let dirs: Vec<&str> = cur_dir.split("/").collect();
    dirs.last().unwrap_or_else(|| panic!("Failed to get current dir. Provided dir: {} is invalid", cur_dir))
}

// TODO: use crate for errors
pub enum ErrorType {
    BUILD,
    EXECUTION,
    CREATION,
    DEPENDENCIES,
    MISC,
    INITIALIZATION,
}

pub fn throw_error<T>(err_type: ErrorType, msg: &str, ctx: Option<String>) -> T {
    let error_lit = "error".red();
    let err = format!("{} {}: {}", get_err_str(&err_type).red(), error_lit, msg);

    println!("\n{err}");

    if let Some(context) = ctx {
        println!("Tip: {}", context);
    }

    process::exit(0);
}

fn get_err_str(err_type: &ErrorType) -> &str {
    match err_type {
        ErrorType::BUILD => "Build",
        ErrorType::EXECUTION => "Execution",
        ErrorType::CREATION => "Project Creation",
        ErrorType::DEPENDENCIES => "Dependencies",
        ErrorType::MISC => "Misc",
        ErrorType::INITIALIZATION => "Initialization",
    }
}

pub fn create_dir(dir: &str) {
    match fs::create_dir(dir) {
        Ok(()) => (),
        Err(err) => throw_error(
ErrorType::CREATION,
     &format!("Failed to create `{}` directory. Please report this on https://github.com/Thepigcat76/surtur/issues", dir),
     Some(format!("{}", err))
        ),
    }
}