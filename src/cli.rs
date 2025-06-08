/// Handling of commands, arguments.
/// Also interacts with config module to
/// gather/store configuration.
use std::{env, path::PathBuf, process::Command};

use anyhow::Context;
use clap::{Parser, Subcommand};

use crate::{
    tool::{config::Config, creator, executor, initiator},
    util::{files::FileHandler, MISSING_CFG},
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

#[derive(Parser, Debug)]
#[command(name = "surtur", version, about = "Tge bzukd systen fir c")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub struct Project {
    pub cfg: Option<Config>,
    pub cur_dir: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the current binary project
    #[command(alias = "r")]
    Run {
        /// Enable debug mode for this project
        #[arg(short, long)]
        debug: bool,

        /// Arguments passed to the program
        #[arg(value_name = "PROGRAM_ARGS", trailing_var_arg = true)]
        program_args: Vec<String>,
    },

    /// Initialize a surtur project in the current directory
    Init,

    /// Build the project into a library or executable
    #[command(alias = "b")]
    Build {
        /// Compile in release mode
        #[arg(short, long)]
        release: bool,

        /// Compile in debug mode
        #[arg(short, long)]
        debug: bool,

        // Hardcoded windows targetting
        #[arg(long)]
        win: bool,
    },

    /// Run a specific or all tests
    Test {
        /// Specify a test name
        #[arg(value_name = "NAME")]
        name: Option<String>,
    },

    /// Create a new project
    Add {
        /// Name for the project
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Remove a dependency
    Remove {
        /// Dependency to remove
        #[arg(value_name = "DEPENDENCY")]
        dependency: String,
    },

    /// Create a new project
    #[command(alias = "n")]
    New {
        /// Name for the project
        #[arg(value_name = "NAME")]
        name: String,

        /// Mark the project as a library
        #[arg(short, long)]
        lib: bool,
    },

    /// Update or install dependencies
    Update {
        /// Force update dependencies even if up to date
        #[arg(short, long)]
        force: bool,
    },

    /// Check for memory leaks using valgrind
    MemCheck,
}

impl Project {
    pub fn try_open() -> anyhow::Result<Self> {
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
        let cli = Cli::parse();
        match cli.command {
            Commands::Run {
                debug: _,
                program_args,
            } => executor::run_c(self, true, program_args),
            Commands::Build { release, debug , win} => executor::build_c(self, debug, false, release, win)
                .context("Failed to build program as executable"),
            Commands::Init => initiator::init_proj(creator::Project::new(&self.cur_dir)),
            Commands::Test { name } => executor::run_test(
                self,
                &match name {
                    Some(tests) => tests,
                    None => "*".into(),
                },
            ),
            Commands::Update { force } => self.cfg.as_ref().context(MISSING_CFG)?.deps.download_deps(force),
            Commands::MemCheck => self.check_mem(),
            Commands::New { name, lib } => creator::Project::new(&PathBuf::from(name)).create(lib),
            _ => {
                println!("{INTRO}");
                Ok(())
            }
        }
    }

    fn check_mem(self) -> anyhow::Result<()> {
        let name = self.cfg.as_ref().unwrap().name.clone();
        executor::build_c(self, true, true, false, false)?;
        let mut cmd = Command::new("valgrind");
        cmd.arg("--leak-check=full");
        cmd.arg("--show-leak-kinds=all");
        cmd.arg(format!("./build/{}", name));
        cmd.spawn()?.wait()?;
        Ok(())
    }
    
}
