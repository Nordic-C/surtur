/*
Handling of commands, arguments.
Interacts with config module to
gather/store configuration.
*/

use std::{env::{self, args}, process::Command};

use maplit::hashmap;

use crate::{builder::Builder, creator::Project};

const INTRO: &str =
r#"
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
    let cmd_tips = hashmap!(
        "uninstall" => "remove",
        "install" => "add",
        "compile" => "build",
        "execute" => "run",
        "create" => "new",
        "package" => "bundle"
    );
    let args: Vec<String> = env::args().collect();

    let first_arg = args.get(1);
    let second_arg = args.get(2);

    let mut matched = false;
    
    match first_arg {
        Some(arg) => match arg.as_str() {
            "run" => run_c(),
            "build" => build_c(),
            "new" => create_proj(match second_arg {
                Some(arg) => arg,
                None => panic!("Missing second arg"),
            }),
            _ => {
                for (key, val) in cmd_tips {
                    if arg.as_str() == key {
                        matched = true;
                        println!("`{}` is not a valid argument. Use `{}` instead", key, val);
                        break;
                    }
                }
                if !matched {
                    println!("`{}` is not a valid argument. Use `help` to see all valid arguments", arg)
                }
            },
        },
        None => println!("{}", INTRO),
    }
}

fn run_c() {
    build_c();
    let command = "./example/build/main.exe";

    let mut cmd = Command::new(command);

    let result = cmd.output();

    match result {
        Ok(output) => {
            if output.status.success() {
                println!(
                    "Command output:\n{}",
                    String::from_utf8_lossy(&output.stdout)
                );
            } else {
                eprintln!(
                    "Command failed with error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}

fn build_c() {
    let mut builder = Builder::new();
    builder.build().expect("Failed to build project");
}

fn create_proj(name: &str) {
    let project = Project::new(name);
    project.create();
}