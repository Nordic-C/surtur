/*
Handling of the project's lua config file.
It includes the lua variable evaluator and all information
related to the project's configuration
*/

use std::{
    fs::{read_to_string, File},
    io::Read,
};

use rlua::{Lua, Table};

use crate::builder::Standard;

pub struct ConfigFile {
    c_std: Standard,
    proj_version: f32,
    dependencies: Vec<String>,
}

impl ConfigFile {
    pub fn from(path: &str) -> Self {
        let mut file = File::open(path).expect("Failed to open file");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("Failed to read file");

        let lua = Lua::new();

        lua.context(|ctx| {
            ctx.load(&buffer).exec().expect("Failed to load context");

            let my_table: Table = ctx.globals().get("Dependencies").expect("Failed to get dependencies");
            println!("{:?}", my_table);

            for key in my_table.pairs::<String, String>() {
                let (k, v) = key.expect("Failed to get key");
                println!("Key: {}, Value: {}", k, v);
            }
        });
        todo!()
    }
}
