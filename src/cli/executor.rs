//! This module provides wrappings around
//! the Compiler for easily running and building
//! everything

use std::{env, fs, path::PathBuf, process::Command};

use anyhow::Context;

use crate::{
    cli::Cli,
    util::{self, MISSING_CFG},
};

use super::{
    compiler::{CompileCtx, Compiler},
    config::ProjType,
};

pub fn run_c(cli: Cli, enable_dbg: bool, args: Option<Vec<&String>>) -> anyhow::Result<()> {
    let cur_dir = cli.cur_dir.clone();
    let root_name = util::root_dir_name(&cur_dir);
    let executable_path = format!(
        "./build/{}",
        root_name.context("Failed to get root name of project")?
    );

    self::build_c(cli, enable_dbg, true, false)?;

    // Create a Command to run the executable
    let mut cmd = Command::new(executable_path);
    if let Some(args) = args {
        cmd.args(args);
    }

    util::run_c_program(&mut cmd, &cur_dir)
}

pub fn build_c(
    cli: Cli,
    enable_dbg: bool,
    direct_execution: bool,
    is_release: bool,
) -> anyhow::Result<()> {
    let mut cfg = cli.cfg.context(MISSING_CFG)?;
    if let Some(sm) = &cfg.scripts {
        sm.pre_exec().context("Failed to run build scripts")?;
    }

    if direct_execution {
        cfg.proj_type = ProjType::Bin;
    } else {
        cfg.proj_type = ProjType::Lib;
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

    let ctx = CompileCtx {
        out_dir: &PathBuf::from("build"),
        root_dir: &cli.cur_dir,
        out_name: root_name,
        excluded: &cfg.excluded,
    };

    compiler.build(ctx, enable_dbg, is_release, false)?;

    if let Some(sm) = &cfg.scripts {
        sm.post_exec().context("Failed to run build scripts")?;
    }

    Ok(())
}

pub fn run_test(cli: Cli, tests: &str) -> anyhow::Result<()> {
    let cfg = cli.cfg.context(MISSING_CFG)?;
    let compiler = Compiler::new(&cli.cur_dir, &cfg)?;

    if fs::metadata("./build").is_err() {
        fs::create_dir("./build").context("Failed to create build directory")?
    }

    if fs::metadata("./build/tests").is_err() {
        fs::create_dir("./build/tests").context("Failed to create build/tests directory")?
    }

    compiler.build_deps()?;

    let root_dir = &env::current_dir()?;

    let ctx = CompileCtx {
        excluded: &cfg.excluded,
        out_dir: &PathBuf::from("build/tests"),
        root_dir,
        out_name: &cfg.name,
    };

    compiler.build(ctx, false, false, true)?;

    env::set_var("SURTUR_TESTS", tests);

    let mut program = Command::new(format!("./build/tests/{}", compiler.root_name));

    util::run_c_program(&mut program, root_dir)
}
