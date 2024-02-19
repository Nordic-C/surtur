/// Handling of building and running the c program with gcc.
/// This inclues functions for
/// building, running, linking and bundling libraries.
use std::{fmt::Display, fs, path::PathBuf, process::Command};

use crate::util;

use super::{
    config::{ConfigFile, ProjType},
    deps::DepManager,
};

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

pub struct Compiler {
    cmd: String,
    deps: DepManager,
    std: Standard,
    proj_type: ProjType,
    proj_dir: PathBuf,
    root_name: String,
}

impl Compiler {
    pub fn new(cur_dir: &str, cfg: ConfigFile) -> Self {
        let root_name = util::root_dir_name(cur_dir);
        Self {
            cmd: cfg.compiler,
            deps: cfg.deps,
            proj_type: cfg.proj_type,
            std: cfg.c_std,
            proj_dir: cur_dir.into(),
            root_name: root_name.into(),
        }
    }

    pub fn build(
        &self,
        root_dir: &PathBuf,
        out_dir: &PathBuf,
        out_name: String,
        comp_type: CompType,
        enable_dbg: bool,
        is_release: bool,
    ) {
        let standard = format!("-std={}", self.std);
        let mut program = Command::new(&self.cmd);
        let src_files = util::get_src_files(&format!("{}/src", root_dir.display()).into());
        let header_files = util::get_header_files(&format!("{}/src", root_dir.display()).into());

        let mut final_src_files = Vec::new();
        for file in src_files {
            let src_name = file.to_string_lossy().to_string();
            let header_name = src_name.replace(".c", ".h");
            let src_name = src_name.split('/').collect::<Vec<&str>>();
            let src_name = src_name
                .last()
                .unwrap_or_else(|| panic!("Path: {} is invalid", file.display()));
            if header_files.contains(&PathBuf::from(header_name)) || *src_name == "main.c" {
                final_src_files.push(file);
            }
        }

        if enable_dbg {
            program.arg("-g");
        } else if is_release {
            program.arg("-o3");
        }

        match comp_type {
            // TODO: linux && macOS file ending
            CompType::Exe => program.args(final_src_files).arg("-o").arg(format!(
                "{}/{}",
                out_dir.display(),
                out_name
            )),
            CompType::Asm => program
                .arg("-S")
                .args(final_src_files)
                .current_dir(format!("{}/build", self.proj_dir.display())),
            CompType::Obj => program
                .arg("-c")
                .args(final_src_files)
                .current_dir(format!("{}/build", self.proj_dir.display())),
        }
        .arg(standard);

        program
            .status()
            .unwrap_or_else(|err| panic!("Failed to compile program: {}", err));
    }

    fn build_lib(&self, root_dir: &PathBuf, out_dir: &PathBuf, out_name: String) {
        let standard = format!("-std={}", self.std);
        let src_files = util::get_src_files(&format!("{}/src", root_dir.display()).into());
        let mut out_names = Vec::new();

        for file in src_files {
            let mut program = Command::new(&self.cmd);
            let name = file
                .to_string_lossy()
                .to_string()
                .split('/')
                .last()
                .unwrap_or_else(|| panic!("Invalid src file path: {}", file.display()))
                .to_string();
            let out_path = format!("build/{}o", &name[..name.len() - 1]);
            program
                .arg("-c")
                .arg(&file)
                .arg("-o")
                .arg(&out_path)
                .arg(&standard);
            program
                .spawn()
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to compile src file: {}, error: {}",
                        &file.display(),
                        err
                    )
                })
                .wait()
                .expect("Failed to wait lol");
            out_names.push(out_path);
        }
        let mut linker = Command::new("ar");
        linker
            .arg("rcs")
            .arg(format!("build/{}.a", out_name))
            .args(out_names);
        linker.spawn().expect("Failed to link library");
    }

    fn link_lib() {}

    fn build_deps(&self) {
        for dep in &self.deps.deps {
            let out_dir = format!("{}/build", &self.proj_dir.display());
            let name = self.deps.deps.get(dep).unwrap().name();
            dbg!("name: {}", &name);
            self.build_lib(&dep.location(), &out_dir.into(), name);
        }
    }
}

/// This module provides wrappings around
/// the Compiler for easily running and building
/// everything
pub mod executor {
    use std::{env, fs, path::PathBuf, process::Command};

    use crate::{
        cli::Cli,
        util::{self, MISSING_CFG},
    };

    use super::{CompType, Compiler};

    pub fn run_c(cli: Cli, enable_dbg: bool) {
        let root_name = util::root_dir_name(&cli.cur_dir);
        let executable_path = format!("./build/{}", root_name);

        self::build_c(cli, CompType::Exe, enable_dbg, false);

        // Create a Command to run the executable
        let mut cmd = Command::new(executable_path);

        match cmd.status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command failed with exit code: {}", status);
                }
            }
            Err(err) => eprintln!("Error: {:?}", err),
        }
    }

    pub fn build_c(cli: Cli, comp_type: CompType, enable_dbg: bool, is_release: bool) {
        let cfg = cli.cfg.unwrap_or_else(|| panic!("{}", MISSING_CFG));
        let compiler = Compiler::new(&cli.cur_dir, cfg);

        let root_name = util::root_dir_name(&cli.cur_dir);
        let executable_path = format!("./build/{}", root_name);

        if fs::metadata("./build").is_err() {
            fs::create_dir("./build").expect("Failed to create build directory")
        }

        compiler.build_deps();

        compiler.build(
            &env::current_dir().unwrap_or_else(|err| panic!("Failed to get cur dir: {}", err)),
            &PathBuf::from("build"),
            root_name.into(),
            comp_type,
            enable_dbg,
            is_release,
        );
    }
}
