/* Creation and initialization of surtur C project */

use std::{
    fs::{self, File},
    io::Write,
};

use git2::{Repository, RepositoryInitOptions};

use crate::util::{throw_error, ErrorType, self};

#[derive(Debug)]
pub struct Project {
    pub root_dir: String,
    pub name: String,
}

const MAIN_FILE_LAYOUT: &str = r#"#include <stdio.h>

int main(void) {
    printf("Hello, World!\n");
}
"#;

impl Project {
    pub fn new(root_dir: &str) -> Self {
        let dirs: Vec<&str> = root_dir.split("/").collect();
        let name = match dirs.last() {
            Some(name) => *name,
            None => throw_error(
                ErrorType::CREATION,
                "Failed to get name of the root directory",
                None,
            ),
        };
        Self {
            root_dir: root_dir.to_string(),
            name: name.to_string(),
        }
    }

    pub fn create(&self) {
        // Rooot dir
        self.create_src_dir(&self.name);

        // Git repo
        self.create_git_repo();

        // Source dir
        self.create_dir("src");

        // Cfg file
        Self::create_cfg_file(&self.root_dir, &self.name);

        // Main file
        Self::create_main_file(&self.root_dir);
    }

    fn create_dir(&self, name: &str) {
        util::create_dir(&format!("{}/{}", self.name, name))
    }

    fn create_src_dir(&self, name: &str) {
        match fs::create_dir(&self.name) {
            Ok(()) => (),
            Err(err) => throw_error(
    ErrorType::CREATION,
         &format!("Failed to create `{}` directory. Please report this on https://github.com/Thepigcat76/surtur/issues", name),
         Some(format!("{}", err))
            ),
        }
    }

    fn create_lib(&self) {}

    fn create_git_repo(&self) {
        // Initialize options for creating the repository.
        let mut opts = RepositoryInitOptions::new();
        opts.external_template(false);

        // Create the Git repository.
        Repository::init_opts(format!("{}", self.name), &opts).expect("Failed to create repo");
    }

    fn get_cfg_file_layout(name: &str) -> String {
        let layout = format!(
            r#"-- versioning
Name = "{}"
Versions = {{
    c = "c17",
    proj = "0.1"
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

    pub fn create_main_file(root_dir: &str) {
        let mut main_file = match File::create(format!("{}/src/main.c", root_dir)) {
            Ok(file) => file,
            Err(err) => throw_error(
    ErrorType::CREATION,
         "Failed to create main file directory. Please report this on https://github.com/Thepigcat76/surtur/issues",
         Some(format!("{}", err))
            ),
        };

        // write content to main file
        match main_file
            .write_all(MAIN_FILE_LAYOUT.as_bytes()) {
                Ok(file) => file,
                Err(err) => throw_error(
        ErrorType::CREATION,
             "Failed to write content to main file. Please report this on https://github.com/Thepigcat76/surtur/issues",
             Some(format!("{}", err))
                ),
            };
    }

    pub fn create_cfg_file(root_dir: &str, root_name: &str) {
        let mut config_file = match File::create(format!("{}/project.lua", root_dir)) {
            Ok(file) => file,
            Err(err) => throw_error(
         ErrorType::CREATION,
         "Failed to create config file. Please report this on https://github.com/Thepigcat76/surtur/issues",
         Some(format!("{}", err))
            ),
        };

        // Write content to cfg file
        match config_file.write_all(Self::get_cfg_file_layout(&root_name).as_bytes()) {
                Ok(()) => (),
                Err(err) => throw_error(
        ErrorType::CREATION,
             "Failed to write content to cfg file. Please report this on https://github.com/Thepigcat76/surtur/issues",
             Some(format!("{}", err))
                ),
            }
    }
}
