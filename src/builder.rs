/*
Handling of building and running the c program with gcc.
This inclues functions for
building, running, linking and bundling libraries.
*/

/*
Handling of compiling, runnning, linking
and bundling of libraries/programs
*/
use std::{process::{Command, Output, Child}, io::Error};

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
    
}

impl Builder {
    pub fn new(root_dir: &str) -> Self {
        let source = format!("{}/src/main.c", root_dir);
        let output = "C:/Users/Admin/programming/rust/surtur/example/build/main.exe";
        let dependencies = Vec::new();
        let mut command = Command::new("gcc");

        command
        .arg(source)
        .arg("-o")
        .arg(output)
        .arg("-std=c17");
        
        Self { command, dependencies, output: output.to_string(), source: source.to_string() }
    }

    pub fn build(&mut self) -> Result<Child, Error> {
        println!("{:?}", self.command);
        let output = self.command.spawn()?;
        Ok(output)
    }
}
