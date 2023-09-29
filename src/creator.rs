/* Creation and initialization of surtur C project */

use std::{
    fs::{self, File},
    io::Write,
};

use git2::{Repository, RepositoryInitOptions};

use crate::util::{throw_error, ErrorType};

pub struct Project {
    root_name: String,
}

const MAIN_FILE_LAYOUT: &str = r#"#include <stdio.h>

int main(void) {
    printf("Hello, World!\n");
    return 0;
}
"#;

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            root_name: name.to_string(),
        }
    }

    pub fn create(&self) {
        // Rooot dir
        match fs::create_dir(format!("{}", self.root_name)) {
            Ok(()) => (),
            Err(err) => throw_error(
    ErrorType::CREATION,
         "Failed to create root directory. Please report this on https://github.com/Thepigcat76/surtur/issues",
         format!("{}", err).as_str()
            ),
        }

        // Git repo
        self.create_git_repo();

        // Source dir
        match fs::create_dir(format!("{}/src", self.root_name)) {
            Ok(()) => (),
            Err(err) => throw_error(
    ErrorType::CREATION,
         "Failed to create `src` directory. Please report this on https://github.com/Thepigcat76/surtur/issues",
         format!("{}", err).as_str()
            ),
        }

        // Build dir
        match fs::create_dir(format!("{}/build", self.root_name)) {
            Ok(()) => (),
            Err(err) => throw_error(
    ErrorType::CREATION,
         "Failed to create `build` directory. Please report this on https://github.com/Thepigcat76/surtur/issues",
         format!("{}", err).as_str()
            ),
        }

        // Cfg file
        let mut config_file = match File::create(format!("{}/project.lua", self.root_name)) {
            Ok(file) => file,
            Err(err) => throw_error(
         ErrorType::CREATION,
         "Failed to create config file. Please report this on https://github.com/Thepigcat76/surtur/issues",
         format!("{}", err).as_str()
            ),
        };

        // Write content to cfg file
        match config_file
            .write_all(Self::get_cfg_file_layout(&self.root_name).as_bytes()) {
                Ok(()) => (),
                Err(err) => throw_error(
        ErrorType::CREATION,
             "Failed to write content to cfg file. Please report this on https://github.com/Thepigcat76/surtur/issues",
             format!("{}", err).as_str()
                ),
            }

        // Main file
        let mut main_file = match File::create(format!("{}/src/main.c", self.root_name)) {
            Ok(file) => file,
            Err(err) => throw_error(
    ErrorType::CREATION,
         "Failed to create main file directory. Please report this on https://github.com/Thepigcat76/surtur/issues",
         format!("{}", err).as_str()
            ),
        };

        // write content to main file
        match main_file
            .write_all(MAIN_FILE_LAYOUT.as_bytes()) {
                Ok(file) => file,
                Err(err) => throw_error(
        ErrorType::CREATION,
             "Failed to write content to main file. Please report this on https://github.com/Thepigcat76/surtur/issues",
             format!("{}", err).as_str()
                ),
            };
    }

    fn create_git_repo(&self) {
        // Initialize options for creating the repository.
        let mut opts = RepositoryInitOptions::new();
        opts.external_template(false);

        // Create the Git repository.
        let repo = Repository::init_opts(format!("{}", self.root_name), &opts)
            .expect("Failed to create repo");

        println!("Repository created at: {:?}", repo.path());
    }

    fn get_cfg_file_layout(name: &str) -> String {
        let layout = format!(
            r#"-- versioning
Name = "{}"
Versions = {{
    ["c"] = "c17",
    ["proj"] = "0.1"
}}
        
-- external dependents
Dependencies = {{
    -- {{ "dependency_name", 0.1 }}
}}
        "#,
            name
        );
        layout
    }
}
