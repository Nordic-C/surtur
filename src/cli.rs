/*
Handling of commands, arguments.
Interacts with config module to
gather/store configuration.
*/

use std::{env, fs::File, process::Command};

use maplit::hashmap;

use crate::{
    builder::{Builder, CompType, Standard},
    config::ConfigFile,
    creator::Project, util,
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

    let path = format!(
        "{}/project.lua",
        cur_dir.to_str().expect("failed to get current directory"),
    );

    let mut file = match File::open(&path) {
        Ok(file) => Some(file),
        Err(_) => None,
    };

    let args: Vec<String> = env::args().collect();

    let first_arg = args.get(1);
    let second_arg = args.get(2);

    let mut matched = false;

    match first_arg {
        Some(arg) => match arg.as_str() {
            // TODO: Create git repo
            "new" => {
                create_proj(match second_arg {
                    Some(arg) => arg,
                    None => panic!("Missing second arg"),
                });
            }
            "run" => {
                let config = ConfigFile::from(&mut file.unwrap());
                run_c(config.c_std);
            }
            "build" => {
                let config = ConfigFile::from(&mut file.unwrap());
                let comp_type = match second_arg {
                    Some(arg) => match arg.as_str() {
                        "-exe" => CompType::EXE,
                        "-asm" => CompType::ASM,
                        "-obj" => CompType::OBJ,
                        "-e" => CompType::EXE,
                        "-a" => CompType::ASM,
                        "-s" => CompType::ASM,
                        "-o" => CompType::OBJ,
                        _ => panic!("Invalid argument"),
                    },
                    None => CompType::EXE,
                };
                build_c(comp_type, config.c_std);
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

fn run_c(std: Standard) {
    build_c(CompType::EXE, std);
    let cur_dir_raw = env::current_dir().expect("Failed to get current directory");
    let cur_dir = cur_dir_raw.to_str().unwrap();
    let root_name = util::root_dir_name(cur_dir);
    let command = format!("./build/{root_name}.exe");

    let mut cmd = Command::new(command);
    cmd.spawn().expect(format!("Failed to run program. Path: {cur_dir}/build/{root_name}.exe").as_str());
}

fn build_c(comp_type: CompType, std: Standard) {
    let cur_dir = env::current_dir().expect("Failed to get current directory");
    let cur_dir_str = cur_dir.to_str().unwrap();
    println!("{}", cur_dir_str);
    let mut builder = Builder::new(cur_dir_str);
    builder
        .build(comp_type, std)
        .expect("Failed to build project");
}

fn create_proj(name: &str) {
    let project = Project::new(name);
    project.create();
}
