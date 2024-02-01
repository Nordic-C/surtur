/// Handling of commands, arguments.
/// Also interacts with config module to
/// gather/store configuration.
pub mod compiler;
pub mod config;
pub mod creator;
pub mod deps;
pub mod initiator;

use std::{collections::HashMap, env, path::PathBuf};

use clap::{arg, command, value_parser, ArgMatches, Command as CCommand};
use clutils::{files::FileHandler, map};

use crate::subcommand;

use self::{
    compiler::{executor, CompType},
    config::ConfigFile,
    creator::Project,
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
    pub cfg: Option<ConfigFile>,
    pub cur_dir: String,
}

impl Cli {
    pub fn new() -> Self {
        let cur_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(_) => todo!(),
        };

        let cur_dir = match cur_dir.to_str() {
            Some(cur_dir) => cur_dir.to_string(),
            None => todo!(),
        };

        let path = format!("{}/project.lua", cur_dir,);

        let cfg = match FileHandler::new(&path) {
            Ok(fh) => Some(ConfigFile::from(fh)),
            Err(_) => None,
        };

        Self { cfg, cur_dir }
    }

    // TODO: add this back
    pub fn get_cmd_tips(&self) -> HashMap<&str, &str> {
        map! [
            "uninstall" => "remove",
            "install" => "add",
            "compile" => "build",
            "execute" => "run",
            "create" => "new",
            "package" => "bundle"
        ]
    }

    pub fn execute(&self) {
        self.match_args()
    }

    fn match_args(&self) {
        match Self::handle_cmd() {
            m if m.subcommand_matches("run").is_some() => executor::run_c(self, false),
            m if m.subcommand_matches("build").is_some() => {
                executor::build_c(self, CompType::Exe, false, false)
            }
            m if m.subcommand_matches("init").is_some() => {
                initiator::init_proj(&Project::new(&self.cur_dir))
            }
            m if m.subcommand_matches("dbg-deps").is_some() => {
                let config = self.cfg.as_ref().expect("Failed to get config file");

                let dep_manager = &config.dependencies;
                dep_manager.init_dep_dir();
                dep_manager.get_dep(0).expect("Failed to get dependency 0");
                dep_manager.get_dep(1).expect("Failed to get dependency 1");
            }
            // Switch this to if let guards once they are stabelized
            m if m.subcommand_matches("new").is_some() => {
                // Unwrap is safe because of .is_some() check
                let name = m
                    .subcommand_matches("new")
                    .unwrap()
                    .get_one::<PathBuf>("NAME");

                match name {
                    Some(name) => Project::new(&name.display().to_string()).create(),
                    None => eprintln!("Failed to create project because of issues with the NAME argument.
                        Please report this issue on github and give additional context: https://github.com/Thepigcat76/surtur/issues"),
                }
            }
            _ => println!("{}", INTRO),
        }
    }

    fn handle_cmd() -> ArgMatches {
        command!()
            .subcommand(CCommand::new("run").about("Run the current binary project"))
            .subcommand(CCommand::new("init").about("Initialize a surtur project in the current directory"))
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
            .subcommand(CCommand::new("dbg-deps").about("Only exists for debugging dependencies. // TODO: remove this"))
            .get_matches()
    }
}
