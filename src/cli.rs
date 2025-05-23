/// Handling of commands, arguments.
/// Also interacts with config module to
/// gather/store configuration.

use std::{env, path::PathBuf, process::Command};

use anyhow::{bail, Context};
use clap::{arg, command, value_parser, ArgMatches, Command as CCommand};

use crate::{
    subcommand, tool::{config::Config, creator::Project, executor, initiator}, util::{files::FileHandler, MISSING_CFG}
};

const INTRO: &str = r#"
This is the Surtur build tool for C

The most important commands are:
- new <name> // create a new surtur C project
- run // compiles and executes your program
- build // compiles your program
- add <name> // adds the specified library
- remove <name> // removes the specified library
- update // Update & install dependencies
- init // initialize a surtur C project
"#;

pub struct Cli {
    pub cfg: Option<Config>,
    pub cur_dir: PathBuf,
}

impl Cli {
    pub fn new() -> anyhow::Result<Self> {
        let cur_dir = env::current_dir()?;

        let path = cur_dir.join("project.lua");

        let fh = FileHandler::new(path.as_path())
            .context(format!("Failed to find path: {path:?}"))
            .ok();
        let cfg = if let Some(fh) = fh {
            Some(Config::parse(&cur_dir, fh)?)
        } else {
            None
        };

        Ok(Self { cfg, cur_dir })
    }

    #[inline]
    pub fn exec(self) -> anyhow::Result<()> {
        self.match_args()
    }

    fn match_args(self) -> anyhow::Result<()> {
        match Self::handle_cmd() {
            m if m.subcommand_matches("run").is_some() => {
                let matches = m.subcommand_matches("run").unwrap();

                let args: Option<Vec<&String>> =
                    matches.get_many("PROGRAM_ARGS").map(|many| many.collect());

                executor::run_c(self, true, args)?
            }
            m if m.subcommand_matches("build").is_some() => {
                let matches = m.subcommand_matches("build").unwrap();

                let enable_dbg = matches.get_flag("debug");
                let is_release = matches.get_flag("release");

                executor::build_c(self, enable_dbg, false, is_release)
                    .context("Failed to build program as executable")?;
            }
            m if m.subcommand_matches("init").is_some() => {
                initiator::init_proj(&Project::new(&self.cur_dir))?;
            }
            m if m.subcommand_matches("test").is_some() => self.run_test(m)?,
            m if m.subcommand_matches("update").is_some() => {
                self.update(m.subcommand_matches("update").unwrap().get_flag("force"))?
            }
            m if m.subcommand_matches("mem-check").is_some() => self.check_mem()?,
            // Switch this to if let guards once they are stabelized
            m if m.subcommand_matches("new").is_some() => Self::new_proj(m)?,
            _ => println!("{}", INTRO),
        }
        Ok(())
    }

    fn handle_cmd() -> ArgMatches {
        command!()
            .subcommand(
                CCommand::new("run")
                .about("Run the current binary project")
                .arg(
                    arg!(-d --debug "Enable debug mode for this project")
                        .required(false)
                )
                .arg(
                    arg!(<PROGRAM_ARGS> ... "Args")
                        .required(false)
                )
            )
            .subcommand(
                CCommand::new("init")
                .about("Initialize a surtur project in the current directory")
            )
            .subcommand(
                CCommand::new("build")
                    .about("Build the project into a library or executable")
                    .arg(
                        arg!(-r --release "Compile the program in release mode (better optimization)")
                            .required(false),
                    )
                    .arg(
                        arg!(-d --debug "Compile the program in debug mode (more advanced debugging capabilities)")
                            .required(false),
                    ),
            )
            .subcommand(
                subcommand!("test", "Run a specific or all tests",
                arg!(<NAME> "Specify a test name").required(false))
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
            ).arg(arg!(-l --lib "Mark the project as a library")))
            .subcommand(CCommand::new("update")
                .about("Update or install dependecies")
                .arg(
                    arg!(-f --force "Force update dependencies, even if there is no new version")
                        .required(false))
            )
            .subcommand(CCommand::new("mem-check")
                .about("Check your program for memory leaks using valgrind")
            )
            .get_matches()
    }

    fn run_test(self, m: ArgMatches) -> anyhow::Result<()> {
        let cmd = m
            .subcommand_matches("test")
            .context("Failed to match subcommand `test`")?;
        let tests = cmd.get_one::<PathBuf>("NAME");
        executor::run_test(
            self,
            &match tests {
                Some(tests) => tests.to_string_lossy().to_string(),
                None => "*".into(),
            },
        )
    }

    fn check_mem(self) -> anyhow::Result<()> {
        let name = self.cfg.as_ref().unwrap().name.clone();
        executor::build_c(self, true, true, false)?;
        let mut cmd = Command::new("valgrind");
        cmd.arg("--leak-check=full");
        cmd.arg("--show-leak-kinds=all");
        cmd.arg(format!("./build/{}", name));
        cmd.spawn()?.wait()?;
        Ok(())
    }

    fn update(&self, forced: bool) -> anyhow::Result<()> {
        let dep_manager = &self.cfg.as_ref().context(MISSING_CFG)?.deps;
        dep_manager.download_deps(forced)
    }

    fn new_proj(m: ArgMatches) -> anyhow::Result<()> {
        let cmd = m.subcommand_matches("new").unwrap();
        // Unwrap is safe because of .is_some() check
        let name = cmd.get_one::<PathBuf>("NAME");
        let is_lib = cmd.get_flag("lib");

        match name {
            Some(name) => Project::new(name).create(is_lib),
            None => bail!("Failed to create project because of issues with the NAME argument"),
        }
    }
}
