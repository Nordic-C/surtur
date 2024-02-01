/// Handling of the project's lua config file.
/// It includes the lua parser and all information
/// related to the project's configuration

use clutils::files::FileHandler;
use rlua::{Lua, Table, Value};
use strum::IntoEnumIterator;

use super::{
    compiler::Standard,
    deps::{Dependency, DepManager},
};

pub struct ConfigFile {
    pub c_std: Standard,
    pub proj_version: String,
    pub dependencies: DepManager,
}

impl ConfigFile {
    pub fn from(file: FileHandler) -> Self {
        let mut dependencies: Vec<Dependency> = Vec::new();
        let mut c_std_str = String::new();
        let mut proj_version = String::new();

        let stds: Vec<Standard> = Standard::iter().collect();
        let mut c_std: Option<Standard> = None;

        let lua = Lua::new();

        lua.context(|ctx| {
            ctx.load(&file.content).exec().expect("Failed to load context");

            // dependencies
            let dep_table: Table = ctx
                .globals()
                .get("Dependencies")
                .expect("Failed to get dependencies");

            // versions
            let versions_table: Table = ctx
                .globals()
                .get("Versions")
                .expect("Failed to get versions");

            versions_table
                .pairs::<String, String>()
                .into_iter()
                .for_each(|pair| {
                    let (key, val) = pair.expect("Failed to get pair");
                    match key.to_lowercase().as_str() {
                        "c" => c_std_str = val,
                        "proj" => proj_version = val,
                        _ => panic!("invalid version entry"),
                    }
                });

            // Iterating over dependencies
            for dep in dep_table.sequence_values::<Table>() {
                let mut version = 0.0;
                let mut origin = String::new();
                for pair in dep
                    .expect("Failed to get table")
                    .sequence_values::<Value>()
                {
                    match pair.expect("Failed to get dependency pair") {
                        Value::Integer(int_value) => {
                            version = int_value as f64;
                        }
                        Value::String(string_value) => {
                            let origin_lit = string_value
                                .to_str()
                                .expect("Failed to convert to str")
                                .to_string();

                            origin = origin_lit;
                        }
                        Value::Number(num_value) => {
                            version = num_value as f64;
                        }
                        _ => {
                            panic!()
                        }
                    }
                }
                let dependency = Dependency::new(&origin, version as f32);
                dependencies.push(dependency);
                println!("{:?}", dependencies);
            }
        });

        // version selection
        for std in stds {
            if c_std_str == std.to_string() {
                c_std = Some(std);
                break;
            }
        }

        Self {
            c_std: match c_std {
                Some(std) => std,
                None => panic!("Invalid C Standard: {:?}", c_std_str),
            },
            proj_version,
            dependencies: DepManager::new(dependencies),
        }
    }
}
