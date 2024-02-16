/// Handling of creating new binary- adn library
/// projects. Includes example layout for both
/// the main.c file and the project.lua config.
/// In addition to that there are also are helper
/// functions for creating all directories and files
use std::{
    fs:: File,
    io::Write,
};

use git2::{Repository, RepositoryInitOptions};

use crate::util;

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
        let dirs: Vec<&str> = root_dir.split('/').collect();
        let name = match dirs.last() {
            Some(name) => *name,
            None => todo!(),
        };
        Self {
            root_dir: root_dir.to_string(),
            name: name.to_string(),
        }
    }

    pub fn create(&self, is_lib: bool) {
        // Root dir
        self.create_root_dir();

        // Git repo
        self.create_git_repo();

        // Source dir
        self.create_dir("src");

        // Cfg file
        Self::create_cfg_file(&self.root_dir, &self.name, is_lib);

        // Main file
        Self::create_main_file(&self.root_dir, is_lib);
    }

    fn create_dir(&self, name: &str) {
        util::create_dir(&format!("{}/{}", self.name, name))
    }

    fn create_root_dir(&self) {
        util::create_dir(&self.name)
    }

    fn create_git_repo(&self) {
        // Initialize options for creating the repository.
        let mut opts = RepositoryInitOptions::new();
        opts.external_template(false);

        // Create the Git repository.
        Repository::init_opts(&self.name, &opts).expect("Failed to create repo");
    }

    fn get_cfg_file_layout(name: &str, lib: bool) -> String {
        format!(
            concat!(
                "\n-- versioning\n",
                "Name = \"{}\"\n",
                "Versions = {{\n",
                "    std = \"c17\",\n",
                "    version = \"0.1\",\n",
                "    type = \"{}\"\n",
                "}}\n",
                "\n-- external dependents\n",
                "Dependencies = {{\n",
                "    -- {{ \"dependency_name\", 0.1 }}\n",
                "}}\n"
            ),
            name,
            if lib { "lib" } else { "bin" }
        )
    }

    pub fn create_main_file(root_dir: &str, is_lib: bool) {
        let mut main_file = File::create(format!(
            "{}/src/{}.c",
            root_dir,
            if is_lib { "lib" } else { "main" }
        ))
        .unwrap_or_else(|err| panic!("Failed to create main file, error: {}", err));

        // write content to main file
        match main_file.write_all(MAIN_FILE_LAYOUT.as_bytes()) {
            Ok(file) => file,
            Err(_) => todo!(),
        }
    }

    pub fn create_cfg_file(root_dir: &str, root_name: &str, lib: bool) {
        let mut config_file = match File::create(format!("{}/project.lua", root_dir)) {
            Ok(file) => file,
            Err(_) => todo!(),
        };

        // Write content to cfg file
        match config_file.write_all(Self::get_cfg_file_layout(root_name, lib).as_bytes()) {
            Ok(()) => (),
            Err(_) => todo!(),
        }
    }
}
