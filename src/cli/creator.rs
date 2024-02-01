/// Handling of creating new binary- adn library
/// projects. Includes example layout for both
/// the main.c file and the project.lua config.
/// In addition to that there are also are helper
/// functions for creating all directories and files
use std::{
    fs::{self, File},
    io::Write,
};

use git2::{Repository, RepositoryInitOptions};

use crate::util;

#[derive(Debug)]
pub struct Project {
    pub root_dir: String,
    pub name: String,
}

// TODO: try using \ to make it look better
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
            None => todo!(),
        };
        Self {
            root_dir: root_dir.to_string(),
            name: name.to_string(),
        }
    }

    pub fn create(&self) {
        // Root dir
        self.create_root_dir(&self.name);

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

    fn create_root_dir(&self, name: &str) {
        match fs::create_dir(&self.name) {
            Ok(()) => (),
            Err(err) => todo!(),
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
        format!(
            concat!(
                "\n-- versioning\n",
                "Name = \"{}\"\n",
                "Versions = {{\n",
                "    c = \"c17\",\n",
                "    proj = \"0.1\"\n",
                "}}\n",
                "\n-- external dependents\n",
                "Dependencies = {{\n",
                "    -- {{ \"dependency_name\", 0.1 }}\n",
                "}}\n"
            ),
            name
        )
    }

    pub fn create_main_file(root_dir: &str) {
        let mut main_file = match File::create(format!("{}/src/main.c", root_dir)) {
            Ok(file) => file,
            Err(err) => todo!(),
        };

        // write content to main file
        match main_file
            .write_all(MAIN_FILE_LAYOUT.as_bytes()) {
                Ok(file) => file,
                Err(err) => todo!(),
            };
    }

    pub fn create_cfg_file(root_dir: &str, root_name: &str) {
        let mut config_file = match File::create(format!("{}/project.lua", root_dir)) {
            Ok(file) => file,
            Err(err) => todo!(),
        };

        // Write content to cfg file
        match config_file.write_all(Self::get_cfg_file_layout(&root_name).as_bytes()) {
                Ok(()) => (),
                Err(err) => todo!(),
            }
    }
}
