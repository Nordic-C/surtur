//! This module provides wrappings around
//! the Compiler for easily running and building
//! everything

use std::{env, fs, path::PathBuf, process::Command};

use anyhow::Context;

use crate::{
    cli::Cli,
    util::{self, MISSING_CFG},
};

use super::compiler::{Compiler, CompType};

pub fn run_c(cli: Cli, enable_dbg: bool, args: Vec<&String>) -> anyhow::Result<()> {
    let cur_dir = cli.cur_dir.clone();
    let root_name = util::root_dir_name(&cur_dir);
    let executable_path = format!(
        "./build/{}",
        root_name.context("Failed to get root name of project")?
    );

    self::build_c(cli, CompType::Exe, enable_dbg, false)?;

    // Create a Command to run the executable
    let mut cmd = Command::new(executable_path);
    cmd.args(args);

    util::run_c_program(&mut cmd, &cur_dir)
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

    let root_dir = &env::current_dir()?;

    compiler.build(
        root_dir,
        &PathBuf::from("build/tests"),
        &compiler.root_name,
        CompType::Exe,
        false,
        false,
        true,
    )?;

    env::set_var("SURTUR_TESTS", tests);

    let mut program = Command::new(format!("./build/tests/{}", compiler.root_name));
    
    util::run_c_program(&mut program, root_dir)
}
