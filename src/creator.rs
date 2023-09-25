/* Creation and initialization of surtur C project */

use std::{fs::{self, File}, io::Write};

pub struct Project {
    root_name: String,
}

const MAIN_FILE_LAYOUT: &str =
r#"#include <stdio.h>

int main(void) {
    printf("Hello, World!");
    return 0;
}
"#;

impl Project {
    pub fn new(name: &str) -> Self {
        Self { root_name: name.to_string() }
    }

    pub fn create(&self) {
        fs::create_dir(format!("{}", self.root_name)).expect("Failed to create root dir");
        fs::create_dir(format!("{}/src", self.root_name)).expect("Failed to create src dir");
        fs::create_dir(format!("{}/build", self.root_name)).expect("Failed to create build dir");
        let mut config_file = File::create(format!("{}/project.lua", self.root_name)).expect("Faile to create project config file");
        config_file.write_all(Self::get_cfg_file_layout(&self.root_name).as_bytes()).expect("Failed to write to file");
        let mut main_file = File::create(format!("{}/src/main.c", self.root_name)).expect("Faile to create project config file");
        main_file.write_all(MAIN_FILE_LAYOUT.as_bytes()).expect("Failed to write main file content")
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

}}
        "#
        , name);
        layout
    }
}
