/// Handling of building and running the c program with gcc.
/// This inclues functions for
/// building, running, linking and bundling libraries.
use std::{fmt::Display, path::PathBuf, process::Command};

use crate::util;

use super::{
    config::{Config, ProjType},
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
    //Asm,
    //Obj,
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
    pub fn new(cur_dir: &str, cfg: Config) -> Self {
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
        tests: bool,
    ) {
        let standard = format!("-std={}", self.std);
        let mut program = Command::new(&self.cmd);
        let src_files = util::get_src_files(&format!("{}/src", root_dir.display()).into());

        if enable_dbg {
            program.arg("-g");
        } else if is_release {
            program.arg("-o3");
        }

        program.arg("-I./deps");

        let mut program = match comp_type {
            // TODO: linux && macOS file ending
            CompType::Exe => {
                program.args(src_files).arg("-o").arg(format!(
                    "{}/{}",
                    out_dir.display(),
                    out_name
                ));
                if !tests {
                    program.arg("-DNOTESTS");
                    dbg!("COMPILING WITH NO TESTS");
                }
                program
            }
        };

        program.arg(standard);

        self.link_lib(&mut program);

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
                .arg("-I./deps")
                .arg(&file)
                .arg("-o")
                .arg(&out_path)
                .arg("-w")
                .arg("-DNOTESTS")
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
            .arg(format!("{}/{}.a", out_dir.display(), out_name))
            .args(out_names);
        linker.spawn().expect("Failed to link library");
    }

    fn build_test(&self, root_dir: &PathBuf) {
        let out_name = &self.root_name;
        let out_dir = format!("{}/build/tests", root_dir.display());
        let standard = format!("-std={}", self.std);
        let mut program = Command::new(&self.cmd);
        let src_files = util::get_src_files(&format!("{}/src", root_dir.display()).into());

        program
            .args(src_files)
            .arg("-o")
            .arg(format!("{}/{}", out_dir, out_name))
            .arg(standard);

        self.link_lib(&mut program);

        program
            .status()
            .unwrap_or_else(|err| panic!("Failed to compile program: {}", err));
    }

    fn link_lib(&self, cmd: &mut Command) {
        cmd.arg("-Lbuild/");
        for dep in &self.deps.deps {
            cmd.arg(format!("-l:{}.a", dep.name()));
        }
    }

    fn build_deps(&self) {
        for dep in &self.deps.deps {
            let out_dir = format!("{}/build", &self.proj_dir.display());
            let name = self.deps.deps.get(dep).unwrap().name();
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
            false,
        );
    }

    pub fn run_test(cli: Cli, tests: &String) {
        let cfg = cli.cfg.unwrap_or_else(|| panic!("{}", MISSING_CFG));
        let compiler = Compiler::new(&cli.cur_dir, cfg);

        if fs::metadata("./build").is_err() {
            fs::create_dir("./build").expect("Failed to create build directory")
        }

        if fs::metadata("./build/tests").is_err() {
            fs::create_dir("./build/tests").expect("Failed to create tests directory")
        }

        compiler.build_deps();

        compiler.build(
            &env::current_dir().unwrap_or_else(|err| panic!("Failed to get cur dir: {}", err)),
            &PathBuf::from("build/tests"),
            compiler.root_name.clone().into(),
            CompType::Exe,
            false,
            false,
            true,
        );

        env::set_var("SURTUR_TESTS", tests);

        let mut program = Command::new(format!("./build/tests/{}", compiler.root_name));
        program.spawn().unwrap_or_else(|err| panic!("{err}"));
    }
}
