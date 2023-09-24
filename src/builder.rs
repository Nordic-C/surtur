/*
Handling of building and running the c program with gcc.
This inclues functions for
building, running, linking and bundling libraries.
*/

/*
Handling of compiling, runnning, linking
and bundling of libraries/programs
*/
use std::{
    io::Error,
    process::{Child, Command, Output},
};

use crate::util;

// TODO: move this to a seperate file
struct Dependency {
    /// Used for finding location of dependency
    name: String,
    /// Used for switching between versions
    version: f32,
}

pub struct Builder {
    command: Command,
    dependencies: Vec<Dependency>,
    output: String,
    source: String,
}

pub enum Standard {
    C89,
    C99,
    C11,
    C17,
    C2X,
    GNU89,
    GNU99,
    GNU11,
    GNU17,
    GNU2X,
}

pub enum CompType {
    EXE,
    ASM,
    OBJ,
}

impl Builder {
    pub fn new(cur_dir: &str) -> Self {
        let root_name = util::root_dir_name(cur_dir);
        let source = format!("{}/src/main.c", cur_dir);
        let output = format!("{}/build/{}", cur_dir, root_name);
        let dependencies = Vec::new();
        let command = Command::new("gcc");

        Self {
            command,
            dependencies,
            output: output.to_string(),
            source: source.to_string(),
        }
    }

    pub fn build(&mut self, comp_type: CompType) -> Result<Child, Error> {
        let mut program = Command::new("gcc");
        let cmd = match comp_type {
            // TODO: linux && macOS file ending
            CompType::EXE => {
                program
                    .arg(&self.source)
                    .arg("-o")
                    .arg(format!("{}.exe", &self.output))
                    .arg("-std=c17")
            }
            CompType::ASM => {
                program
                    .arg("-S")
                    .arg(&self.source)
                    .arg("-o")
                    .arg(format!("{}.s", &self.output))
                    .arg("-std=c17")
            },
            CompType::OBJ => {
                program
                    .arg("-c")
                    .arg(&self.source)
                    .arg("-o")
                    .arg(format!("{}.o", &self.output))
                    .arg("-std=c17")
            },
        };

        println!("{:?}", cmd);
        let output = cmd.spawn()?;
        Ok(output)
    }
}
