use std::process;

use colored::Colorize;

pub fn root_dir_name(cur_dir: &str) -> &str {
    let dirs: Vec<&str> = cur_dir.split("/").collect();
    dirs[dirs.len() - 1]
}

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
