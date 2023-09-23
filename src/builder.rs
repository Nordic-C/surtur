/*
Handling of building and running the c program with gcc.
This inclues functions for
building, running, linking and bundling libraries.
*/

use std::{process::{Command, Output}, io::Error};

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

impl Builder {
    pub fn new() -> Self {
        let source = "C:/Users/Admin/programming/rust/surtur/example/src/main.c";
        let output = "C:/Users/Admin/programming/rust/surtur/example/build/main.exe";
        let dependencies = Vec::new();
        let mut command = Command::new("gcc");

        command.arg(source).arg("-o").arg(output);
        
        Self { command, dependencies, output: output.to_string(), source: source.to_string() }
    }

    pub fn build(&mut self) -> Result<Output, Error> {
        println!("{:?}", self.command);
        let output = self.command.output()?;
        Ok(output)
    }
}
