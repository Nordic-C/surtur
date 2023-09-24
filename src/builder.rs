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
    collections::HashMap,
    io::Error,
    process::{Child, Command, Output},
};

use maplit::hashmap;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn build(&mut self, comp_type: CompType, std: Standard) -> Result<Child, Error> {
        let standards = Self::get_standards();
        let standard =format!("-std={}", &standards[&std]);
        let program = &mut self.command;
        let cmd = match comp_type {
            // TODO: linux && macOS file ending
            CompType::EXE => program
                .arg(&self.source)
                .arg("-o")
                .arg(format!("{}.exe", &self.output)),
            CompType::ASM => program
                .arg("-S")
                .arg(&self.source)
                .arg("-o")
                .arg(format!("{}.s", &self.output)),
            CompType::OBJ => program
                .arg("-c")
                .arg(&self.source)
                .arg("-o")
                .arg(format!("{}.o", &self.output)),
        }.arg(standard);

        println!("{:?}", cmd);
        let output = cmd.spawn()?;
        Ok(output)
    }

    fn get_standards() -> HashMap<Standard, String> {
        let standards = hashmap! {
            Standard::C89 => String::from("c89"),
            Standard::C99 => String::from("c99"),
            Standard::C11 => String::from("c11"),
            Standard::C17 => String::from("c17"),
            Standard::C2X => String::from("c2x"),
            Standard::GNU89 => String::from("gnu89"),
            Standard::GNU99 => String::from("gnu99"),
            Standard::GNU11 => String::from("gnu11"),
            Standard::GNU17 => String::from("gnu17"),
            Standard::GNU2X => String::from("gnu2x"),
        };
        standards
    }
}
