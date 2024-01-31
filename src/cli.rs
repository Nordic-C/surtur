/// Handling of commands, arguments.
/// Interacts with config module to
/// gather/store configuration.
use std::{env, fs, process::Command, thread, time::Duration};

use clap::{arg, command, value_parser, ArgMatches, Command as CCommand};
use clutils::{files::FileHandler, map};
use colored::Colorize;

use crate::{
    compiler::{self, CompType, Compiler},
    config::ConfigFile,
    creator::Project,
    initiator, subcommand,
    tips::*,
    util::{self, throw_error, ErrorType},
};

const INTRO: &str = r#"
This is the Surtur build tool for C

The most important commands are:
- new <name> // create a new surtur C project
- run // compiles and executes your program
- build // compiles your program
- help // use for additional help
- add <name> // adds the specified library
- remove <name> // removes the specified library
- update // use this when making changes to the project.lua file
- init // initialize a surtur C project
"#;

pub struct Cli {
    args: Vec<String>,
    pub cfg: Option<ConfigFile>,
    pub cur_dir: String,
}

impl Cli {
    pub fn new() -> Self {
        let cur_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(_) => throw_error(
                ErrorType::MISC,
                "Failed to get current directory.
        Please report this issue here https://github.com/Thepigcat76/surtur/issues",
                None,
            ),
        };

        let cur_dir = match cur_dir.to_str() {
            Some(cur_dir) => cur_dir.to_string(),
            None => throw_error(
                ErrorType::MISC,
                "Failed to convert current directory to string.
        Please report this issue here https://github.com/Thepigcat76/surtur/issues",
                None,
            ),
        };

        let path = format!("{}/project.lua", cur_dir,);

        let cfg = match FileHandler::new(&path) {
            Ok(fh) => Some(ConfigFile::from(fh)),
            Err(_) => None,
        };

        Self {
            cfg,
            cur_dir,
            args: env::args().collect(),
        }
    }

    pub fn execute(&self) {
        let cmd_tips = map! [
            "uninstall" => "remove",
            "install" => "add",
            "compile" => "build",
            "execute" => "run",
            "create" => "new",
            "package" => "bundle"
        ];

        let first_arg = self.args.get(1);
        let second_arg = self.args.get(2);

        let mut matched = false;

        match first_arg {
            Some(arg) => match arg.as_str() {
                "new" => {
                    Project::new(match second_arg {
                        Some(arg) => arg,
                        None => throw_error(
                            ErrorType::CREATION,
                            "Failed to set project name",
                            Some(get_tip(Tip::MissingProjName)),
                        ),
                    })
                    .create();
                }
                "run" => match second_arg {
                    Some(arg) => match arg.as_str() {
                        "-dbg" | "-d" => compiler::executor::run_c(&self, true),
                        _ => throw_error(
                            ErrorType::EXECUTION,
                            "Invalid argument for running the program",
                            Some(get_tip(Tip::InvalidRunArg)),
                        ),
                    },
                    None => compiler::executor::run_c(&self, false),
                },
                "build" => {
                    let mut actual_args = self.args.clone();
                    actual_args.remove(0);

                    let mut is_release = false;
                    let mut comp_type = CompType::Exe;
                    actual_args.iter().for_each(|arg| match arg.as_str() {
                        "-exe" | "-e" => comp_type = CompType::Exe,
                        "-asm" | "-a" | "-s" => comp_type = CompType::Asm,
                        "-obj" | "-o" => comp_type = CompType::Obj,
                        "-release" | "-r" => is_release = true,
                        _ => throw_error(
                            ErrorType::BUILD,
                            "Invalid argument",
                            Some(get_tip(Tip::InvalidBuildArg)),
                        ),
                    });
                    dbg!("{:?}, {}", &actual_args, is_release);
                    compiler::executor::build_c(&self, comp_type, false, is_release);
                }
                "init" => {
                    let root_dir = match env::current_dir() {
                        Ok(root) => match root.to_str() {
                            Some(str_root) => str_root.to_string(),
                            None => throw_error(
                                ErrorType::INITIALIZATION,
                                "Failed to convert root directory to a string",
                                None,
                            ),
                        },
                        Err(_) => throw_error(
                            ErrorType::INITIALIZATION,
                            "Failed to get root directory",
                            None,
                        ),
                    };
                    let proj = Project::new(&root_dir);

                    dbg!("{:?}", &proj);

                    initiator::init_proj(&proj);
                }
                "add" => {
                    todo!()
                }
                "help" => {
                    todo!()
                }
                "dbg-deps" => {
                    let config = self.cfg.as_ref().expect("Failed to get config file");

                    let dep_manager = &config.dependencies;
                    dep_manager.init_dep_dir();
                    dep_manager.get_dep(0).expect("Failed to get dependency 0");
                    dep_manager.get_dep(1).expect("Failed to get dependency 1");
                }
                _ => {
                    for (key, val) in cmd_tips {
                        if arg.as_str() == key {
                            matched = true;
                            eprintln!("`{}` is not a valid argument. Use `{}` instead", key, val);
                            break;
                        }
                    }
                    if !matched {
                        eprintln!(
                            "`{}` is not a valid argument. Use `help` to see all valid arguments",
                            arg
                        )
                    }
                }
            },
            None => println!("{}", INTRO),
        }
    }

    fn match_cmd(arg_matches: ArgMatches) {
        todo!()
    }

    fn handle_cmd() -> ArgMatches {
        command!()
            .subcommand(CCommand::new("run").about("Run the current binary project"))
            .subcommand(
                CCommand::new("build")
                    .about("Build the project into a library or executable")
                    .arg(arg!(-s --asm "Compile the program to assembly").required(false))
                    .arg(arg!(-o --obj "Compile the program to an object file").required(false))
                    .arg(
                        arg!(-x --exe "Compile the program to an exectuable (default)").required(false),
                    )
                    .arg(
                        arg!(-r --release "Compile the program in release mode (better optimization)")
                            .required(false),
                    ),
            )
            .subcommand(subcommand!(
                "add",
                "Create a new project",
                arg!(<NAME> "Name for the project")
            ))
            .subcommand(subcommand!(
                "remove",
                "remove a dependency",
                arg!(<DEPENDENCY> "dependency to remove")
            ))
            .subcommand(subcommand!(
                "new",
                "create a new project",
                arg!(<NAME> "name for the project")
            ))
            .get_matches()
    }
}
