/// Handling of building and running the c program with gcc.
/// This inclues functions for
/// building, running, linking and bundling libraries.
use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;

use crate::util;

use super::{
    config::{Config, ProjType, Properties},
    deps::DepManager,
};

// files to exclude when compiling a c lib by deafult
pub const DEFAULT_LIB_EXCLUDE: &str = "main.c";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Standard {
    C89,
    C99,
    C11,
    C17,
    C2X,
    C23,
    Gnu89,
    Gnu99,
    Gnu11,
    Gnu17,
    Gnu2X,
    Gnu23,
}

impl Standard {
    pub fn from_str(c_std: &str) -> Option<Standard> {
        match c_std {
            "c89" => Some(Standard::C89),
            "c99" => Some(Standard::C99),
            "c11" => Some(Standard::C11),
            "c17" => Some(Standard::C17),
            "c2x" => Some(Standard::C2X),
            "c23" => Some(Standard::C23),
            "gnu89" => Some(Standard::Gnu89),
            "gnu99" => Some(Standard::Gnu99),
            "gnu11" => Some(Standard::Gnu11),
            "gnu17" => Some(Standard::Gnu17),
            "gnu2x" => Some(Standard::Gnu2X),
            "gnu23" => Some(Standard::Gnu23),
            _ => None
        }
    }
}

impl Display for Standard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Standard::C89 => "c89",
            Standard::C99 => "c99",
            Standard::C11 => "c11",
            Standard::C17 => "c17",
            Standard::C2X => "c2x",
            Standard::C23 => "c23",
            Standard::Gnu89 => "gnu89",
            Standard::Gnu99 => "gnu99",
            Standard::Gnu11 => "gnu11",
            Standard::Gnu17 => "gnu17",
            Standard::Gnu2X => "gnu2x",
            Standard::Gnu23 => "gnu23",
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
    libs: &'c HashSet<String>,
    props: &'c Properties,
    proj_dir: &'c PathBuf,
    pub root_name: &'c str,
}

pub struct CompileCtx<'ctx> {
    pub out_dir: &'ctx Path,
    pub out_name: &'ctx str,
    pub root_dir: &'ctx Path,
    pub excluded: &'ctx HashSet<PathBuf>,
}

impl<'c> Compiler<'c> {
    pub fn new(cur_dir: &'c PathBuf, cfg: &'c Config) -> anyhow::Result<Self> {
        let root_name =
            util::root_dir_name(cur_dir).context("Failed to get root name of project")?;
        Ok(Self {
            cmd: &cfg.props.compiler,
            dm: &cfg.deps,
            props: &cfg.props,
            proj_dir: cur_dir,
            libs: &cfg.libraries,
            root_name,
        })
    }

    #[inline(always)]
    pub fn build(
        &self,
        ctx: CompileCtx<'c>,
        enable_dbg: bool,
        is_release: bool,
        tests: bool,
    ) -> anyhow::Result<()> {
        match self.props.proj_type {
            ProjType::Lib => self.build_lib(ctx),
            ProjType::Bin => self.build_exe(ctx, enable_dbg, is_release, tests),
        }
    }

    pub fn build_exe(
        &self,
        ctx: CompileCtx<'c>,
        enable_dbg: bool,
        is_release: bool,
        tests: bool,
    ) -> anyhow::Result<()> {
        let standard = format!("-std={}", self.props.c_std);
        let mut program = Command::new(self.cmd);
        let mut src_files = util::get_src_files(&ctx.root_dir.join("src"));
        src_files.retain(|e| !ctx.excluded.contains(e));

        if enable_dbg {
            program.arg("-g");
        } else if is_release {
            program.arg("-o3");
        }

        program
            .args(src_files)
            .arg("-o")
            .arg(ctx.out_dir.join(ctx.out_name));

        if !tests {
            program.arg("-DNOTESTS");
        }

        println!("command: {:#?} {:#?}", self.cmd, program.get_args());

        program.arg(standard);

        self.link_lib(&mut program).context("Failed to link program to build executable")?;

        program
            .status()
            .context("Failed to build executable")
            .map(|_| ())
    }

    pub fn build_lib(&self, ctx: CompileCtx<'c>) -> anyhow::Result<()> {
        let standard = format!("-std={}", self.props.c_std);
        let mut src_files = util::get_src_files(&ctx.root_dir.join("src"));
        src_files.remove(&ctx.root_dir.join("src").join(DEFAULT_LIB_EXCLUDE));
        src_files.retain(|e| !ctx.excluded.contains(e));
        let mut out_names = Vec::new();

        if src_files.is_empty() {
            return Ok(());
        }

        // TODO: Sort all object files into directory

        for file in src_files {
            let mut program = Command::new(self.cmd);
            // TODO: Clean this up
            let name = file.file_name().unwrap().to_string_lossy().to_string();
            let out_path = ctx.out_dir.join(&name[..name.len() - 2]);
            program
                .arg("-c")
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
            .arg(ctx.out_dir.join(ctx.out_name))
            .args(out_names);
        linker.spawn().context("Failed to link library")?;
        Ok(())
    }

    pub fn link_lib(&self, cmd: &mut Command) -> anyhow::Result<()> {
        cmd.arg("-Lbuild/");
        for dep in &self.dm.deps {
            let name = dep.name()?;
            cmd.arg(format!("-l:{name}/{name}.a"));
        }

        for lib in self.libs {
            cmd.arg(format!("-l{lib}"));
        }

        Ok(())
    }

    pub fn build_deps(&self) -> anyhow::Result<()> {
        for dep in &self.dm.deps {
            let out_dir = self.proj_dir.join("build").join(dep.name()?);
            if !out_dir.exists() {
                fs::create_dir(&out_dir)?;
            }
            let mut name = dep.name()?;
            name.push_str(".a");
            let cfg = dep.config()?;
            let ctx = CompileCtx {
                out_dir: &out_dir,
                out_name: &name,
                root_dir: &dep.location()?,
                excluded: &cfg.excluded,
            };
            self.build_lib(ctx)
                .context(format!("Failed to build library {}", name))?;
        }
        Ok(())
    }

}
