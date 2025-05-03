/// Handling of creating new binary- adn library
/// projects. Includes example layout for both
/// the main.c file and the project.lua config.
/// In addition to that there are also are helper
/// functions for creating all directories and files
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Context;
use git2::{Repository, RepositoryInitOptions};

use crate::util::{self, DEFAULT_COMPILER};

#[derive(Debug)]
pub struct Project<'p> {
    pub root_dir: &'p PathBuf,
    pub name: String,
}

const MAIN_FILE_LAYOUT: &str = r#"#include <stdio.h>

int main(void) {
    printf("Hello, World!\n");
}
"#;

const LIB_MAIN_FILE_LAYOUT: &str = r#"// Run debugging and testing code here. Not included in release build

int main(void) {
    printf("Hello, i am being debugged :)\n");
}
"#;

const LIB_LIB_FILE_LAYOUT: &str = r#"// Example function implementations for the header

#include <stdio.h>
#include "../include/lib.h"

void say_hello() {
    printf("Hello!\n");
}

int square(int number) {
    return number * number;
}
"#;

const HEADER_FILE_LAYOUT: &str = r#"// Example functions you want to export through a header

void say_hello();

int square(int number);
"#;

const GITIGNORE_LAYOUT: &str = "build/\n";

impl<'p> Project<'p> {
    pub fn new(root_dir: &'p PathBuf) -> Self {
        let name = root_dir.file_name().unwrap().to_string_lossy().to_string();
        Self { root_dir, name }
    }

    pub fn create(&self, is_lib: bool) -> anyhow::Result<()> {
        // Root dir
        self.create_root_dir()?;

        // Git repo
        self.create_git_repo()?;

        // Source dir
        self.create_dir("src")?;

        // .gitignore file
        Self::create_gitignore(self.root_dir)?;

        // Cfg file
        Self::create_cfg_file(self.root_dir, &self.name, is_lib)?;

        // Main file
        Self::create_main_file(self.root_dir, is_lib)?;

        if is_lib {
            // Include dir if necessary
            self.create_dir("include")?;
            // Example header if necessary
            Self::create_header_file(self.root_dir)?;
            Self::create_lib_file(self.root_dir)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn create_dir(&self, name: &str) -> anyhow::Result<()> {
        util::create_dir(&format!("{}/{}", self.name, name))
    }

    #[inline(always)]
    fn create_root_dir(&self) -> anyhow::Result<()> {
        util::create_dir(&self.name)
    }

    fn create_git_repo(&self) -> anyhow::Result<()> {
        // Initialize options for creating the repository.
        let mut opts = RepositoryInitOptions::new();
        opts.external_template(false);

        // Create the Git repository.
        Repository::init_opts(&self.name, &opts)
            .context("Failed to create git repository")
            .map(|_| ())
    }

    fn get_cfg_file_layout(name: &str, lib: bool) -> String {
        format!(
            concat!(
                "-- properties\n",
                "Name = \"{}\"\n",
                "Props = {{\n",
                "    std = \"c17\",\n",
                "    version = \"0.1\",\n",
                "    type = \"{}\",\n",
                "    compiler = \"{}\",\n",
                "}}\n",
                "{}",
                "\n-- external dependenciess\n",
                "Dependencies = {{\n",
                "    -- {{ \"https://github.com/Surtur-Team/surtests\", 0.1 }}\n",
                "}}\n"
            ),
            name,
            if lib { "lib" } else { "bin" },
            DEFAULT_COMPILER,
            if lib {
                "\n-- C files that should not be compiled manually (don't have a header)\n-- lib.c is excluded here if your project is a library\nExclude = {\n    \"lib.c\",\n}\n"
            } else {
                ""
            }
        )
    }

    pub fn create_main_file(root_dir: &Path, is_lib: bool) -> anyhow::Result<()> {
        let mut main_file =
            File::create(root_dir.join("src/main.c")).context("Failed to create main file")?;

        // write content to main file
        main_file
            .write_all(
                if is_lib {
                    LIB_MAIN_FILE_LAYOUT
                } else {
                    MAIN_FILE_LAYOUT
                }
                .as_bytes(),
            )
            .context("Failed to write example code to main.c file")
    }

    pub fn create_lib_file(root_dir: &Path) -> anyhow::Result<()> {
        let mut lib_file =
            File::create(root_dir.join("src/lib.c")).context("Failed to create lib file")?;

        // write content to main file
        lib_file
            .write_all(LIB_LIB_FILE_LAYOUT.as_bytes())
            .context("Failed to write example code to main.c file")
    }

    pub fn create_header_file(root_dir: &Path) -> anyhow::Result<()> {
        let mut header_file =
            File::create(root_dir.join("include/lib.h")).context("Failed to create header file")?;

        // write content to main file
        header_file
            .write_all(HEADER_FILE_LAYOUT.as_bytes())
            .context("Failed to write example code to header file")
    }

    pub fn create_cfg_file(root_dir: &Path, root_name: &str, lib: bool) -> anyhow::Result<()> {
        let mut config_file =
            File::create(root_dir.join("project.lua")).context("Failed to create config gile")?;

        // Write content to cfg file
        config_file
            .write_all(Self::get_cfg_file_layout(root_name, lib).as_bytes())
            .context("Failed to write config to project.lua")
    }

    pub fn create_gitignore(root_dir: &Path) -> anyhow::Result<()> {
        let mut gitignore_file = File::create(root_dir.join(".gitignore"))
            .context("Failed to create .gitignore file")?;

        // Write content to .gitignore file
        gitignore_file
            .write_all(GITIGNORE_LAYOUT.as_bytes())
            .context("Failed to write gitignore content to .gitignore")
    }
}
