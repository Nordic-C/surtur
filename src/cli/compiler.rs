/// Handling of building and running the c program with gcc.
/// This inclues functions for
/// building, running, linking and bundling libraries.
use std::{fmt::Display, path::PathBuf, process::Command};

use anyhow::Context;

use crate::util;

use super::{
    config::{Config, ProjType},
    deps::DepManager,
};

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
pub const STANDARDS: [Standard; 10] = [
    Standard::C89,
    Standard::C99,
    Standard::C11,
    Standard::C17,
    Standard::C2X,
    Standard::Gnu89,
    Standard::Gnu99,
    Standard::Gnu11,
    Standard::Gnu17,
    Standard::Gnu2X,
];

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

pub struct Compiler<'c> {
    cmd: &'c String,
    dm: &'c DepManager,
    std: Standard,
    proj_type: ProjType,
    proj_dir: &'c PathBuf,
    root_name: &'c str,
}

impl<'c> Compiler<'c> {
    pub fn new(cur_dir: &'c PathBuf, cfg: &'c Config) -> anyhow::Result<Self> {
        let root_name =
            util::root_dir_name(cur_dir).context("Failed to get root name of project")?;
        Ok(Self {
            cmd: &cfg.compiler,
            dm: &cfg.deps,
            proj_type: cfg.proj_type,
            std: cfg.c_std,
            proj_dir: cur_dir,
            root_name,
        })
    }

    pub fn build(
        &self,
        root_dir: &PathBuf,
        out_dir: &PathBuf,
        out_name: &str,
        comp_type: CompType,
        enable_dbg: bool,
        is_release: bool,
        tests: bool,
    ) -> anyhow::Result<()> {
        match self.proj_type {
            ProjType::Lib => self.build_lib(root_dir, out_dir, out_name),
            ProjType::Bin => self.build_exe(
                root_dir, out_dir, out_name, comp_type, enable_dbg, is_release, tests,
            ),
        }
    }

    pub fn build_exe(
        &self,
        root_dir: &PathBuf,
        out_dir: &PathBuf,
        out_name: &str,
        comp_type: CompType,
        enable_dbg: bool,
        is_release: bool,
        tests: bool,
    ) -> anyhow::Result<()> {
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
                }
                program
            }
        };

        program.arg(standard);

        self.link_lib(&mut program)?;

        program
            .status()
            .context("Failed to build executable")
            .map(|_| ())
    }

    fn build_lib(
        &self,
        root_dir: &PathBuf,
        out_dir: &PathBuf,
        out_name: &str,
    ) -> anyhow::Result<()> {
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
                .context(format!("Invalid src file path: {}", file.display()))?
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
                .context(format!("Failed to compile src file: {}", &file.display()))?
                .wait()
                .context("Failed to wait for src file to compile")?;
            out_names.push(out_path);
        }
        let mut linker = Command::new("ar");
        linker
            .arg("rcs")
            .arg(format!("{}/{}.a", out_dir.display(), out_name))
            .args(out_names);
        linker.spawn().context("Failed to link library")?;
        Ok(())
    }

    fn link_lib(&self, cmd: &mut Command) -> anyhow::Result<()> {
        cmd.arg("-Lbuild/");
        for dep in &self.dm.deps {
            cmd.arg(format!("-l:{}.a", dep.name()?));
        }
        Ok(())
    }

    fn build_deps(&self) -> anyhow::Result<()> {
        for dep in &self.dm.deps {
            let out_dir = format!("{}/build", &self.proj_dir.display());
            let name = dep.name()?;
            self.build_lib(&dep.location()?, &out_dir.into(), &name)
                .context(format!("Failed to build library {}", name))?;
        }
        Ok(())
    }
}

/// This module provides wrappings around
/// the Compiler for easily running and building
/// everything
pub mod executor {
    use std::{env, fs, path::PathBuf, process::Command};

    use anyhow::Context;

    use crate::{
        cli::Cli,
        util::{self, MISSING_CFG},
    };

    use super::{CompType, Compiler};

    pub fn run_c(cli: Cli, enable_dbg: bool) -> anyhow::Result<()> {
        let root_name = util::root_dir_name(&cli.cur_dir);
        let executable_path = format!(
            "./build/{}",
            root_name.context("Failed to get root name of project")?
        );

        self::build_c(cli, CompType::Exe, enable_dbg, false)?;

        // Create a Command to run the executable
        let mut cmd = Command::new(executable_path);

        match cmd.status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command failed with exit code: {}", status);
                }
            }
            Err(err) => {
                return Err(err)
                    .context("Failed to run the c program. Execution of the program failed.")
            }
        }
        Ok(())
    }

    pub fn build_c(
        cli: Cli,
        comp_type: CompType,
        enable_dbg: bool,
        is_release: bool,
    ) -> anyhow::Result<()> {
        let cfg = cli.cfg.context(format!("{}", MISSING_CFG))?;
        if let Some(sm) = &cfg.scripts {
            sm.pre_exec().context("Failed to run build scripts")?;
        }
        let compiler = Compiler::new(&cli.cur_dir, &cfg)?;

        let root_name =
            util::root_dir_name(&cli.cur_dir).context("Failed to get root name of project")?;

        if fs::metadata("./build").is_err() {
            fs::create_dir("./build").context("Failed to create build directory")?
        }

        compiler
            .build_deps()
            .context("Failed to build dependencies")?;

        compiler.build(
            &env::current_dir().context(format!("Failed to get cur dir"))?,
            &PathBuf::from("build"),
            root_name.into(),
            comp_type,
            enable_dbg,
            is_release,
            false,
        )?;

        if let Some(sm) = &cfg.scripts {
            sm.post_exec().context("Failed to run build scripts")?;
        }

        Ok(())
    }

    pub fn run_test(cli: Cli, tests: &str) -> anyhow::Result<()> {
        let cfg = cli.cfg.context(format!("{}", MISSING_CFG))?;
        let compiler = Compiler::new(&cli.cur_dir, &cfg)?;

        if fs::metadata("./build").is_err() {
            fs::create_dir("./build").context("Failed to create build directory")?
        }

        if fs::metadata("./build/tests").is_err() {
            fs::create_dir("./build/tests").context("Failed to create build/tests directory")?
        }

        compiler.build_deps()?;

        compiler.build(
            &env::current_dir()?,
            &PathBuf::from("build/tests"),
            &compiler.root_name,
            CompType::Exe,
            false,
            false,
            true,
        )?;

        env::set_var("SURTUR_TESTS", tests);

        let mut program = Command::new(format!("./build/tests/{}", compiler.root_name));
        program.spawn().context("Failed to run tests").map(|_| ())
    }
}
