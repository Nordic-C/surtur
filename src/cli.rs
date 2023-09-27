/*
Handling of commands, arguments.
Interacts with config module to
gather/store configuration.
*/

use std::{
    env,
    fs::{self, File},
    process::Command,
};

use colored::Colorize;
use maplit::hashmap;

use crate::{
    builder::{Builder, CompType, Standard},
    config::ConfigFile,
    creator::Project,
    tips::*,
    util::{self, throw_error, ErrorType},
};

const INTRO: &str = r#"
This is the Surtur build tool for C

The most important commands are:
- new // create a new surtur C project
- run // compiles and executes your program
- build // compiles your program
- help // use for additional help
- add <string> // adds the specified library
- remove <string> // removes the specified library
- update // use this when making changes to the project.lua file
- init // initialize a surtur C project
"#;

pub fn execute() {
    let cmd_tips = hashmap! {
        "uninstall" => "remove",
        "install" => "add",
        "compile" => "build",
        "execute" => "run",
        "create" => "new",
        "package" => "bundle"
    };
    let cur_dir = env::current_dir().expect("Failed to get current directory");

    let cur_dir_str = cur_dir.to_str().expect("failed to get current directory");

    let path = format!("{}\\project.lua", cur_dir_str,);

    let mut file = match File::open(&path) {
        Ok(file) => Some(file),
        Err(_) => None,
    };

    let args: Vec<String> = env::args().collect();

    let first_arg = args.get(1);
    let second_arg = args.get(2);

    let mut matched = false;

    let blue_line = "|".bright_blue();

    let missing_cfg_file = format!(
        r#"
    {} Could not locate config file at {}
    {} 
    {} Use 
    {} {}> {} init
    {} To create a new config file
    "#,
        blue_line,
        path,
        blue_line,
        blue_line,
        blue_line,
        cur_dir_str,
        "surtur".yellow(),
        blue_line,
    );

    match first_arg {
        Some(arg) => match arg.as_str() {
            // TODO: Create git repo
            "new" => {
                create_proj(match second_arg {
                    Some(arg) => arg,
                    None => throw_error(
                        ErrorType::CREATION,
                        "Failed to set project name",
                        &get_tip(Tip::MissingProjName),
                    ),
                });
            }
            "run" => {
                let config = ConfigFile::from(match &mut file {
                    Some(cfg_file) => cfg_file,
                    None => throw_error(
                        ErrorType::EXECUTION,
                        missing_cfg_file.as_str(),
                        "Failed to find config file",
                    ),
                });
                match second_arg {
                    Some(arg) => match arg.as_str() {
                        "-dbg" => run_c(config.c_std, true),
                        "-d" => run_c(config.c_std, true),
                        _ => throw_error(
                            ErrorType::EXECUTION,
                            "Invalid argument for running the program",
                            &get_tip(Tip::InvalidRunArg),
                        ),
                    },
                    None => run_c(config.c_std, false),
                }
            }
            "build" => {
                let mut actual_args = Vec::new();
                for (index, arg) in args.iter().enumerate() {
                    if index > 1 {
                        actual_args.push(arg)
                    }
                }
                let config = ConfigFile::from(match &mut file {
                    Some(cfg_file) => cfg_file,
                    None => throw_error(
                        ErrorType::BUILD,
                        missing_cfg_file.as_str(),
                        "Failed to find config file",
                    ),
                });
                let mut is_release = false;
                let mut comp_type = CompType::EXE;
                for arg in &actual_args {
                    match arg.as_str() {
                        "-exe" | "-e" => comp_type = CompType::EXE,
                        "-asm" | "-a" | "-s" => comp_type = CompType::ASM,
                        "-obj" | "-o" => comp_type = CompType::OBJ,
                        "-release" | "-r" => is_release = true,
                        _ => throw_error(
                            ErrorType::BUILD,
                            "Invalid argument",
                            &get_tip(Tip::InvalidBuildArg),
                        ),
                    }
                }
                println!("{:?}, {}", &actual_args, is_release);
                build_c(comp_type, config.c_std, false, is_release);
            }
            _ => {
                for (key, val) in cmd_tips {
                    if arg.as_str() == key {
                        matched = true;
                        println!("`{}` is not a valid argument. Use `{}` instead", key, val);
                        break;
                    }
                }
                if !matched {
                    println!(
                        "`{}` is not a valid argument. Use `help` to see all valid arguments",
                        arg
                    )
                }
            }
        },
        None => println!("{}", INTRO),
    }
}

use std::thread;

use std::time::Duration;

// TODO: Extremly hacky please fix
fn run_c(std: Standard, enable_dbg: bool) {
    let cur_dir_raw = env::current_dir().expect("Failed to get current directory");
    let cur_dir = match cur_dir_raw.to_str() {
        Some(cur_dir) => cur_dir,
        None => throw_error(ErrorType::MISC, "Failed to convert current directory to string.
        Please report this issue here https://github.com/Thepigcat76/surtur/issues", "__None__"),
    };
    let root_name = util::root_dir_name(cur_dir);
    let executable_path = format!("./build/{}.exe", root_name);

    {
        let mut file_available = true;

        fs::remove_file(format!("build/{}.exe", root_name))
            .expect("Failed to remove old executable");

        while file_available {
            if fs::metadata(&executable_path).is_err() {
                file_available = false;
            } else {
                // Sleep for a short duration before checking again
                thread::sleep(Duration::from_millis(500)); // 500 milliseconds
            }
        }
    }

    build_c(CompType::EXE, std, enable_dbg, false);

    let mut file_available = false;

    while !file_available {
        if fs::metadata(&executable_path).is_ok() {
            file_available = true;
        } else {
            // Sleep for a short duration before checking again
            thread::sleep(Duration::from_millis(500)); // 500 milliseconds
        }
    }

    if file_available {
        // Create a Command to run the executable
        let mut cmd = Command::new(&executable_path);
        cmd.current_dir("./build"); // Set the working directory

        match cmd.status() {
            Ok(status) => {
                if status.success() {
                    println!("Program executed successfully.");
                } else {
                    eprintln!("Command failed with exit code: {}", status);
                }
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    } else {
        eprintln!("Timed out waiting for the executable file to become available.");
    }
}

fn build_c(comp_type: CompType, std: Standard, enable_dbg: bool, is_release: bool) {
    let cur_dir = env::current_dir().expect("Failed to get current directory");
    let cur_dir_str = match cur_dir.to_str() {
        Some(cur_dir) => cur_dir,
        None => throw_error(ErrorType::MISC, "Failed to convert current directory to string.
        Please report this issue here https://github.com/Thepigcat76/surtur/issues", "__None__"),
    };
    println!("{}", cur_dir_str);
    let mut builder = Builder::new(cur_dir_str);
    builder
        .build(comp_type, std, enable_dbg, is_release)
        .expect("Failed to build project");
}

fn create_proj(name: &str) {
    let project = Project::new(name);
    project.create();
}
