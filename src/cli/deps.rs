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

use std::{error::Error, fmt::Display};

use git2::Repository;

use crate::util;

#[derive(Debug, Default)]
pub struct DepManager {
    dependencies: Vec<Dependency>,
}

#[derive(Debug)]
pub struct Dependency {
    version: f32,
    origin: String,
}

impl Dependency {
    pub fn new(origin: &str, version: f32) -> Self {
        let origin = origin.to_string();
        let split_path: Vec<&str> = origin.split('.').collect();
        let origin = match split_path.last() {
            Some(last) => match *last {
                "git" => origin.to_string(),
                _ => {
                    let mut origin = origin.to_string();
                    origin.push_str(".git");
                    origin
                }
            }
            _ => origin.to_string(),
        };
        Self {
            version,
            origin,
        }
    }

    pub fn get_name(&self) -> String {
        let split_path: Vec<&str> = self.origin.split('/').collect();
        let mut name = match split_path.last() {
            Some(name) => name.to_string(),
            None => panic!("Invalid origin {}", self.origin),
        };
        for _ in 0..=3 {
            name.pop();
        }
        name
    }
}

impl DepManager {
    pub fn new(dependencies: Vec<Dependency>) -> Self {
        Self { dependencies }
    }

    pub fn init_dep_dir(&self) {
        util::create_dir("deps");
    }

    /// Downloads the dependency into your projects depndency directoy
    pub fn get_dep(&self, index: usize) -> Result<(), impl Error> {
        let dep = match self.dependencies.get(index) {
            Some(dep) => dep,
            None => {
                return Err(NoDepError(format!(
                    "The dependency at index: {} is out of bounds",
                    index
                )))
            }
        };
        let url = &dep.origin;
        if let Err(err) = Repository::clone(url, format!("deps/{}", dep.get_name())) {
            eprintln!("{}", err)
        }
        Ok(())
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
