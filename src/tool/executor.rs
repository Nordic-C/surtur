//! This module provides wrappings around
//! the Compiler for easily running and building
//! everything

use std::{env, fs, path::PathBuf, process::Command};

use anyhow::Context;

use crate::{
    cli::Project,
    util::{self, MISSING_CFG},
};

use super::{
    compiler::{CompileCtx, Compiler},
    config::ProjType,
};

pub fn run_c(project: Project, enable_dbg: bool, args: Vec<String>) -> anyhow::Result<()> {
    let cur_dir = project.cur_dir.clone();
    let root_name = util::root_dir_name(&cur_dir);
    let executable_path = format!(
        "./build/{}",
        root_name.context("Failed to get root name of project")?
    );

    self::build_c(project, enable_dbg, true, false, false)?;

    // Create a Command to run the executable
    let mut cmd = Command::new(executable_path);
    cmd.args(args);

    util::run_c_program(&mut cmd, &cur_dir)
}

pub fn build_c(
    proj: Project,
    enable_dbg: bool,
    direct_execution: bool,
    is_release: bool,
    win: bool,
) -> anyhow::Result<()> {
    let mut cfg = proj.cfg.context(MISSING_CFG)?;
    if let Some(sm) = &cfg.scripts {
        sm.pre_exec().context("Failed to run build scripts")?;
    }

    if direct_execution {
        cfg.props.proj_type = ProjType::Bin;
    }

    if win {
        cfg.props.compiler = String::from("x86_64-w64-mingw32-gcc");
    }

    let compiler = Compiler::new(&proj.cur_dir, &cfg)?;

    let mut root_name = util::root_dir_name(&proj.cur_dir)
        .context("Failed to get root name of project")?
        .to_string();

    let out_path = PathBuf::from("build");

    if cfg.props.proj_type == ProjType::Lib {
        root_name.push_str(".a");
    }

    if fs::metadata("./build").is_err() {
        fs::create_dir("./build").context("Failed to create build directory")?
    }

    compiler
        .build_deps()
        .context("Failed to build dependencies")?;

    let ctx = CompileCtx {
        out_dir: &out_path,
        root_dir: &proj.cur_dir,
        out_name: &root_name,
        excluded: &cfg.excluded,
    };

    let _ = fs::remove_file(format!("build/{}", ctx.out_name));

    let _ = fs::remove_file(format!("build/{}.exe", ctx.out_name));

    compiler.build(ctx, enable_dbg, is_release, false, win)?;

    if let Some(sm) = &cfg.scripts {
        sm.post_exec()
            .context("Failed to run post build process scripts")?;
    }

    Ok(())
}

pub fn run_test(proj: Project, tests: &str) -> anyhow::Result<()> {
    let mut cfg = proj.cfg.context(MISSING_CFG)?;
    cfg.props.proj_type = ProjType::Bin;
    let compiler = Compiler::new(&proj.cur_dir, &cfg)?;

    let build_dir = PathBuf::from("build");

    if !build_dir.exists() {
        fs::create_dir(&build_dir).context("Failed to create build directory")?
    }

    let tests_dir = build_dir.join("tests");

    if !tests_dir.exists() {
        fs::create_dir(&tests_dir).context("Failed to create build/tests directory")?
    }

    compiler.build_deps()?;

    let ctx = CompileCtx {
        excluded: &cfg.excluded,
        out_dir: &tests_dir,
        root_dir: &proj.cur_dir,
        out_name: &cfg.name,
    };

    compiler.build(ctx, true, false, true, false)?;

    env::set_var("SURTUR_TESTS", tests);

    let mut program = Command::new(tests_dir.join(cfg.name));

    util::run_c_program(&mut program, &proj.cur_dir)
}
