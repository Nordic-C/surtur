/*
Handling of commands, arguments.
Interacts with config module to
gather/store configuration.
*/

use std::{env, process::Command};

use crate::builder::Builder;

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
    let args: Vec<String> = env::args().collect();

    let first_arg = args.get(1);
    
    match first_arg {
        Some(arg) => match arg.as_str() {
            "run" => run_c(),
            "build" => build_c(),
            _ => todo!(),
        },
        None => print!("{}", INTRO),
    }
}

fn run_c() {
    build_c();
    let command = "./example/build/main.exe";

    let mut child = Command::new(command);

    let result = child.output();

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