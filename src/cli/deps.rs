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
use std::{error::Error, fmt::Display, path::PathBuf};

use git2::Repository;

use crate::util;

#[derive(Debug, Default)]
pub struct DepManager {
    pub deps: Vec<Dependency>,
}

#[derive(Debug)]
pub struct Dependency {
    _version: f32,
    origin: String,
}

impl DepManager {
    pub fn new(dependencies: Vec<Dependency>) -> Self {
        Self { deps: dependencies }
    }

    pub fn init_dep_dir(&self) {
        util::create_dir("deps");
    }

    /// Downloads the dependency into your projects depndency directoy
    pub fn download_deps(&self) {
        for dep in &self.deps {
            let url = &dep.origin;
            if let Err(err) = Repository::clone(url, format!("deps/{}", dep.name())) {
                eprintln!("{}", err)
            }
        }
    }

    pub fn dep_locations(&self) -> Vec<PathBuf> {
        self.deps.iter().map(|dep| format!("deps/{}", dep.name()).into()).collect()
    }
}

impl Dependency {
    pub fn new(origin: &str, version: f32) -> Self {
        let origin = match &origin[origin.len() - 4..] {
            ".git" => origin.to_string(),
            _ => {
                let mut origin = origin.to_string();
                origin.push_str(".git");
                origin
            }
        };
        Self {
            _version: version,
            origin,
        }
    }

    pub fn name(&self) -> String {
        let split_path: Vec<&str> = self.origin.split('/').collect();
        let mut name = match split_path.last() {
            Some(name) => name.to_string(),
            None => panic!("Invalid origin {}", self.origin),
        };
        name[..name.len() - 4].into()
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
