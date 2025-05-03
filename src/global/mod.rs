//! Module responsible for global file system operations
//! like installing dependencies and saving configurations

pub mod extensions;

use std::{env, fs, path::PathBuf};

use anyhow::Context;
use dirs::home_dir;
use git2::Repository;

use crate::tool::deps::Dependency;

pub const SURTUR_HOME: &str = "SURTUR_HOME";

/// This creates the .surtur directory if it does not exist yet
pub(super) fn init_dir() -> anyhow::Result<()> {
    let home = home_dir().context("Failed")?;
    let surtur_path = home.join(".surtur");
    env::set_var(SURTUR_HOME, &surtur_path);
    if !surtur_path.exists() {
        fs::create_dir(&surtur_path)?;
        let deps_path = surtur_path.join("deps");
        if !deps_path.exists() {
            fs::create_dir(deps_path)?;
        }
    }
    Ok(())
}

pub(super) fn download_dep(dep: &Dependency, forced: bool) -> anyhow::Result<()> {
    let url = &dep.origin;
    let dep_path = PathBuf::from(env::var(SURTUR_HOME)?)
        .join("deps")
        .join(dep.name()?);
    if !dep_path.exists() {
        if let Err(err) = Repository::clone(url, dep_path) {
            eprintln!("{}", err);
        }
    } else if dep_path.exists() && forced {
        fs::remove_dir_all(&dep_path)?;
        Repository::clone(url, dep_path)?;
    }
    Ok(())
}
