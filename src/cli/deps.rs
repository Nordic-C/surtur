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
use std::{collections::HashSet, env, error::Error, fmt::Display, path::PathBuf};

use anyhow::bail;

use crate::{global, util::files::FileHandler};

use super::config::Config;

#[derive(Debug, Default)]
pub struct DepManager {
    pub deps: HashSet<Dependency>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub _version: String,
    pub origin: String,
}

impl DepManager {
    pub fn new(dependencies: HashSet<Dependency>) -> Self {
        Self { deps: dependencies }
    }

    /// Downloads the dependency into your projects depndency directoy
    pub fn download_deps(&self, forced: bool) -> anyhow::Result<()> {
        for dep in &self.deps {
            global::download_dep(dep, forced)?;
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
        let surtur_home = PathBuf::from(env::var(global::SURTUR_HOME)?);
        let deps_path = surtur_home.join("deps");
        Ok(deps_path.join(self.name()?))
    }

    pub fn config(&self) -> anyhow::Result<Config> {
        let location = self.location()?;
        let cfg_path = location.join("project.lua");
        let file = FileHandler::new(&cfg_path)?;
        Config::parse(&location, file)
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
