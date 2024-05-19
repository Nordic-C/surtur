/// Handling of building and running the c program with gcc.
/// This inclues functions for
/// building, running, linking and bundling libraries.
use std::{collections::HashSet, fmt::Display, path::PathBuf, process::Command};

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
    pub root_name: &'c str,
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

    #[inline(always)]
    pub fn build(
        &self,
        excluded: &HashSet<PathBuf>,
        root_dir: &PathBuf,
        out_dir: &PathBuf,
        out_name: &str,
        enable_dbg: bool,
        is_release: bool,
        tests: bool,
    ) -> anyhow::Result<()> {
        match self.proj_type {
            ProjType::Lib => self.build_lib(excluded, root_dir, out_dir, out_name),
            ProjType::Bin => self.build_exe(
                excluded, root_dir, out_dir, out_name, enable_dbg, is_release, tests,
            ),
        }
    }

    pub fn build_exe(
        &self,
        excluded: &HashSet<PathBuf>,
        root_dir: &PathBuf,
        out_dir: &PathBuf,
        out_name: &str,
        enable_dbg: bool,
        is_release: bool,
        tests: bool,
    ) -> anyhow::Result<()> {
        let standard = format!("-std={}", self.std);
        let mut program = Command::new(&self.cmd);
        let mut src_files = util::get_src_files(&root_dir.join("src"));
        src_files.retain(|e| !excluded.contains(e));

        if enable_dbg {
            program.arg("-g");
        } else if is_release {
            program.arg("-o3");
        }

        program
            .args(src_files)
            .arg("-o")
            .arg(format!("{}/{}", out_dir.display(), out_name));

        if !tests {
            program.arg("-DNOTESTS");
        }

        program.arg(standard);

        self.link_lib(&mut program)?;

        program
            .status()
            .context("Failed to build executable")
            .map(|_| ())
    }

    pub fn build_lib(
        &self,
        excluded: &HashSet<PathBuf>,
        root_dir: &PathBuf,
        out_dir: &PathBuf,
        out_name: &str,
    ) -> anyhow::Result<()> {
        let standard = format!("-std={}", self.std);
        let mut src_files = util::get_src_files(&root_dir.join("src"));
        src_files.remove(&root_dir.join("src").join("lib.c"));
        src_files.retain(|e| !excluded.contains(e));
        let mut out_names = Vec::new();

        if src_files.is_empty() {
            return Ok(());
        }

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

    pub fn link_lib(&self, cmd: &mut Command) -> anyhow::Result<()> {
        cmd.arg("-Lbuild/");
        for dep in &self.dm.deps {
            cmd.arg(format!("-l:{}.a", dep.name()?));
        }
        Ok(())
    }

    pub fn build_deps(&self) -> anyhow::Result<()> {
        for dep in &self.dm.deps {
            let out_dir = format!("{}/build", &self.proj_dir.display());
            let name = dep.name()?;
            let cfg = dep.config()?;
            self.build_lib(&cfg.excluded, &dep.location()?, &out_dir.into(), &name)
                .context(format!("Failed to build library {}", name))?;
        }
        Ok(())
    }
}
