/// Handling of building and running the c program with gcc.
/// This inclues functions for
/// building, running, linking and bundling libraries.
use std::{
    fmt::Display,
    fs,
    io::Error,
    process::{Child, Command},
};

use crate::util;

use super::{config::ProjType, deps::DepManager};

pub struct Compiler {
    command: Command,
    deps: DepManager,
    output: String,
    source: String,
    root_dir: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum::EnumIter)]
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

impl Display for Standard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Standard::C89 => "c89",
            Standard::C99 => "c99",
            Standard::C11 => "c11",
            Standard::C17 => "c17",
            Standard::C2X => "c2x",
            Standard::Gnu89 => "gnu89",
            Standard::Gnu99 => "gnu99",
            Standard::Gnu11 => "gnu11",
            Standard::Gnu17 => "gnu17",
            Standard::Gnu2X => "gnu2x",
        })
    }
}

pub enum CompType {
    Exe,
    Asm,
    Obj,
}

impl Compiler {
    pub fn new(cur_dir: &str, proj_type: &ProjType) -> Self {
        let root_name = util::root_dir_name(cur_dir);
        let source = format!("{}/src/{}.c", cur_dir, proj_type);
        let output = format!("{}/build/{}", cur_dir, root_name);
        let deps = Vec::new();
        let command = Command::new("gcc");
        Self {
            command,
            deps: DepManager::new(deps),
            output,
            source,
            root_dir: cur_dir.into(),
        }
    }

    pub fn build(
        &mut self,
        comp_type: CompType,
        std: Standard,
        enable_dbg: bool,
        is_release: bool,
    ) -> Result<Child, Error> {
        let standard = format!("-std={}", std);
        let program = &mut self.command;
        let mut src_files = Vec::new();
        if let Ok(entries) = fs::read_dir(format!("{}/src", &self.root_dir)) {
            for file in entries.flatten() {
                let name = file.file_name().to_string_lossy().to_string();
                let ending = &name[name.len() - 2..];
                if ending == ".c" {
                    src_files.push(format!("{}/src/{}", self.root_dir, name))
                }
            }
        }

        if enable_dbg {
            program.arg("-g");
        } else if is_release {
            program.arg("-o3");
        }

        match comp_type {
            // TODO: linux && macOS file ending
            CompType::Exe => program.args(src_files).arg("-o").arg(&self.output),
            CompType::Asm => program
                .arg("-S")
                .args(src_files)
                .current_dir(format!("{}/build", self.root_dir)),
            CompType::Obj => program
                .arg("-c")
                .args(src_files)
                .current_dir(format!("{}/build", self.root_dir)),
        }
        .arg(standard);

        let output = program.spawn()?;
        Ok(output)
    }
}

/// This module provides wrappings around
/// the Compiler for easily running and building
/// everything
pub mod executor {
    use std::{fs, process::Command, thread, time::Duration};

    use crate::{cli::Cli, util};

    use super::{CompType, Compiler};

    pub fn run_c(cli: &Cli, enable_dbg: bool) {
        let root_name = util::root_dir_name(&cli.cur_dir);
        let executable_path = format!("./build/{}", root_name);

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
                let mut cmd = Command::new(executable_path);
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
        let mut compiler = Compiler::new(&cli.cur_dir, &cli.cfg.proj_type);

        let root_name = util::root_dir_name(&cli.cur_dir);
        let executable_path = format!("./build/{}", root_name);

        match fs::metadata(&executable_path) {
            Ok(_) => {
                let mut program = Command::new("rm");
                let cmd = program.arg(&executable_path);
                let mut child = cmd.spawn().expect("Failed to spawn child");
                child.wait().expect("Failed to get exitstatus");
            }
            Err(_) => {
                if fs::metadata("./build").is_err() {
                    fs::create_dir("./build").expect("Failed to create build directory")
                }
            }
        }
        compiler
            .build(comp_type, cli.cfg.c_std, enable_dbg, is_release)
            .expect("Failed to build project");
    }
}
