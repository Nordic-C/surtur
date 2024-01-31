/// Handling of building and running the c program with gcc.
/// This inclues functions for
/// building, running, linking and bundling libraries.

use std::{
    collections::HashMap,
    io::Error,
    process::{Child, Command},
};

use clutils::map;

use crate::{util, deps::DepManager};

pub struct Compiler {
    command: Command,
    deps: DepManager,
    output: String,
    source: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Standard {
    C89,
    C99,
    C11,
    C17,
    C2X,
    Gnu89,
    Gnu99,
    Gnu11,
    Gnu17,
    Gnu2X,
}

pub enum CompType {
    Exe,
    Asm,
    Obj,
}

impl Compiler {
    pub fn new(cur_dir: &str) -> Self {
        let root_name = util::root_dir_name(cur_dir);
        let source = format!("{}/src/main.c", cur_dir);
        let output = format!("{}/build/{}", cur_dir, root_name);
        let deps = Vec::new();
        let command = Command::new("gcc");
        Self {
            command,
            deps: DepManager::new(deps),
            output: output.to_string(),
            source: source.to_string(),
        }
    }

    pub fn build(
        &mut self,
        comp_type: CompType,
        std: Standard,
        enable_dbg: bool,
        is_release: bool,
    ) -> Result<Child, Error> {
        let standards = Self::get_standards();
        let standard = format!("-std={}", &standards[&std]);
        let program = &mut self.command;

        if enable_dbg {
            program.arg("-g");
        } else if is_release {
            program.arg("-o3");
        }

        match comp_type {
            // TODO: linux && macOS file ending
            CompType::Exe => program
                .arg(&self.source)
                .arg("-o")
                .arg(format!("{}", &self.output)),
            CompType::Asm => program
                .arg("-S")
                .arg(&self.source)
                .arg("-o")
                .arg(format!("{}.s", &self.output)),
            CompType::Obj => program
                .arg("-c")
                .arg(&self.source)
                .arg("-o")
                .arg(format!("{}.o", &self.output)),
        }
        .arg(standard);

        let output = program.spawn()?;
        Ok(output)
    }

    pub fn get_standards() -> HashMap<Standard, String> {
        let standards = map! {
            Standard::C89 => String::from("c89"),
            Standard::C99 => String::from("c99"),
            Standard::C11 => String::from("c11"),
            Standard::C17 => String::from("c17"),
            Standard::C2X => String::from("c2x"),
            Standard::Gnu89 => String::from("gnu89"),
            Standard::Gnu99 => String::from("gnu99"),
            Standard::Gnu11 => String::from("gnu11"),
            Standard::Gnu17 => String::from("gnu17"),
            Standard::Gnu2X => String::from("gnu2x")
        };
        standards
    }
}

/// This module provides wrappings around
/// the Compiler for easily running and building
/// everything
pub mod executor {
    use std::{fs, process::Command, thread, time::Duration};

    use colored::Colorize;

    use crate::{cli::Cli, util::{self, throw_error, ErrorType}};

    use super::{CompType, Compiler};

    pub fn run_c(cli: &Cli, enable_dbg: bool) {
        let root_name = util::root_dir_name(&cli.cur_dir);
        let executable_path = format!("./build/{}", root_name);

        {
            let mut program = Command::new("rm");
            let cmd = program.arg(&executable_path);
            let mut child = cmd.spawn().expect("Failed to spawn child");
            child.wait().expect("Failed to get exitstatus");
        }

        self::build_c(cli, CompType::Exe, enable_dbg, false);

        let mut file_available = false;

        while !file_available {
            match fs::metadata(&executable_path) {
                Ok(_) => file_available = true,
                // Wait for executable to be available
                Err(_) => thread::sleep(Duration::from_millis(100)),
            }
        }

        match file_available {
            true => {
                // Create a Command to run the executable
                let mut cmd = Command::new(format!("{}", &executable_path));
                cmd.output().expect("Failed to run executable");

                match cmd.status() {
                    Ok(status) => {
                        if !status.success() {
                            eprintln!("Command failed with exit code: {}", status);
                        }
                    }
                    Err(err) => eprintln!("Error: {:?}", err),
                }
            }
            false => eprintln!("Timed out waiting for the executable file to become available."),
        }
    }

    pub fn build_c(cli: &Cli, comp_type: CompType, enable_dbg: bool, is_release: bool) {
        let blue_line = "|".bright_blue();
        let path = format!("{}/project.lua", cli.cur_dir);

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
            cli.cur_dir,
            "surtur".yellow(),
            blue_line,
        );
        let mut builder = Compiler::new(&cli.cur_dir);
        let cfg = match &cli.cfg {
            Some(cfg) => cfg,
            None => throw_error(
                ErrorType::EXECUTION,
                "Missing project config file",
                Some(missing_cfg_file),
            ),
        };
        builder
            .build(comp_type, cfg.c_std, enable_dbg, is_release)
            .expect("Failed to build project");
    }
}