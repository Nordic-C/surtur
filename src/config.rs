/*
 Handling of the project's lua config file.
 It includes the lua variable evaluator and all information
 related to the project's configuration
*/

use std::{
    fs::File,
    io::Read,
};

use rlua::{Lua, Table, Value};

use crate::{builder::{Standard, Builder}, manager::Dependency};

pub struct ConfigFile {
    pub c_std: Standard,
    pub proj_version: String,
    pub dependencies: Vec<Dependency>,
}

impl ConfigFile {
    pub fn from(file: &mut File) -> Self {
        let mut buffer = String::new();

        let mut dependencies: Vec<Dependency> = Vec::new();
        let mut c_std_str = String::new();
        let mut proj_version = String::new();

        let stds = Builder::get_standards();
        let mut c_std: Option<&Standard> = None;

        file.read_to_string(&mut buffer)
            .expect("Failed to read file");

        let lua = Lua::new();

        lua.context(|ctx| {
            ctx.load(&buffer).exec().expect("Failed to load context");

            let dep_table: Table = ctx.globals().get("Dependencies").expect("Failed to get dependencies");
            let versions_table: Table = ctx.globals().get("Versions").expect("Failed to get versions");

            versions_table.pairs::<String, String>().into_iter().for_each(|pair| {
                let (key, val) = pair.expect("Failed to get pair");
                match key.to_lowercase().as_str() {
                    "c" => c_std_str = val,
                    "proj" => proj_version = val,
                    _ => panic!("invalid version entry")
                }
            });

            for dep in dep_table.sequence_values::<Table>() {
                for pair in dep.expect("Failed to get table").sequence_values::<Value>() {
                    let index = &pair.clone().expect("Failed to get pair").type_name();
                    match pair.expect("Failed to get dependency pair") {
                        Value::Integer(int_value) => {
                            println!("Index {}: Integer {}", index, int_value);
                        }
                        Value::String(string_value) => {
                            println!("Index {}: String '{}'", index, string_value.to_str().expect("Failed to get str"));
                        }
                        Value::Number(num_value) => {
                            println!("Index {}: Number {}", index, num_value);
                        }
                        _ => {
                            println!("Index {}: Unknown data type", index);
                        }
                    }
                }
            }
        });

        stds.iter().for_each(|(key, val)| {
            if &c_std_str == val {
                c_std = Some(key);
            }
        });

        if c_std == None {
            panic!("Invalid C Standard: {:?}", c_std_str)
        }

        Self { c_std: *c_std.unwrap(), proj_version, dependencies }
    }
}
