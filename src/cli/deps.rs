/// Handling of project dependencies and
/// utility functions for it.
/// It's main part is the dependency manager
/// for managing the dependencies of your
/// project. It stores all of your project's
/// dependencies.
///
/// Individula dependencies are in the Dependency
/// struct and store basic information about the
/// specific dependency
use std::{collections::HashSet, error::Error, fmt::Display, fs, path::PathBuf};

use anyhow::bail;
use git2::Repository;

use crate::util;

#[derive(Debug, Default)]
pub struct DepManager {
    pub deps: HashSet<Dependency>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Dependency {
    _version: String,
    origin: String,
}

impl DepManager {
    pub fn new(dependencies: HashSet<Dependency>) -> Self {
        Self { deps: dependencies }
    }

    pub fn init_dep_dir(&self) -> anyhow::Result<()> {
        if fs::metadata("deps").is_err() {
            util::create_dir("deps")?;
        }
        bail!("Failed to create depedency directory")
    }

    /// Downloads the dependency into your projects depndency directoy
    pub fn download_deps(&self) -> anyhow::Result<()> {
        for dep in &self.deps {
            let url = &dep.origin;
            if let Err(err) = Repository::clone(url, format!("deps/{}", dep.name()?)) {
                eprintln!("{}", err)
            }
        }
        Ok(())
    }
}

impl Dependency {
    pub fn new(origin: &str, version: &str) -> Self {
        let origin = match &origin[origin.len() - 4..] {
            ".git" => origin.to_string(),
            _ => {
                let mut origin = origin.to_string();
                origin.push_str(".git");
                origin
            }
        };
        Self {
            _version: version.into(),
            origin,
        }
    }

    pub fn name(&self) -> anyhow::Result<String> {
        let split_path: Vec<&str> = self.origin.split('/').collect();
        let name = match split_path.last() {
            Some(name) => name.to_string(),
            None => bail!("Invalid origin {}", self.origin),
        };
        Ok(name[..name.len() - 4].into())
    }

    pub fn location(&self) -> anyhow::Result<PathBuf> {
        Ok(format!("deps/{}", self.name()?).into())
    }
}

#[derive(Debug)]
pub struct NoDepError(String);

impl Display for NoDepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for NoDepError {}
