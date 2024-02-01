use std::fmt::Display;

/// Handling of the project's lua config file.
/// It includes the lua parser and all information
/// related to the project's configuration
use clutils::files::FileHandler;
use rlua::{Lua, Table, Value};
use strum::IntoEnumIterator;

use super::{
    compiler::Standard,
    deps::{DepManager, Dependency},
};

pub struct ConfigFile {
    pub c_std: Standard,
    pub proj_version: String,
    pub proj_type: ProjType,
    pub dependencies: DepManager,
}

#[derive(Debug)]
pub enum ProjType {
    Lib,
    Bin,
}

impl Display for ProjType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ProjType::Lib => "lib",
            ProjType::Bin => "bin",
        })
    }
}

const MISSING_VER: &str = concat!(
    "The projects version is missing.",
            "This value needs to be declared in your project.lua file in the Props table with the name `version`\n Example: Props = {{ version = \"0.1\" }}");

impl ConfigFile {
    pub fn from(file: FileHandler) -> Self {
        let mut dependencies: Vec<Dependency> = Vec::new();
        let mut c_std_str = String::from("c17");
        let mut proj_version = None;
        let mut proj_type = ProjType::Lib;

        let stds: Vec<Standard> = Standard::iter().collect();
        let mut c_std: Option<Standard> = None;

        let lua = Lua::new();

        lua.context(|ctx| {
            ctx.load(&file.content)
                .exec()
                .expect("Failed to load context");

            // dependencies
            let dep_table: Table = ctx
                .globals()
                .get("Dependencies")
                .expect("Failed to get dependencies");

            // versions
            let props_table: Table = ctx.globals().get("Props").expect("Failed to get versions");

            props_table
                .pairs::<String, String>()
                .for_each(|pair| {
                    let (key, val) = pair.expect("Failed to get pair");
                    match key.to_lowercase().as_str() {
                        "std" => c_std_str = val,
                        "version" => proj_version = Some(val),
                        "type" => {
                            proj_type = match val.as_str() {
                                "lib" => ProjType::Lib,
                                "bin" => ProjType::Bin,
                                _ => panic!("`{}` is not a valid value for the projects type. Valid types are: `lib` and `bin`", val),
                            }
                        }
                        key => panic!("invalid version entry: {}", key),
                    }
                });

            // Iterating over dependencies
            for dep in dep_table.sequence_values::<Table>() {
                let mut version = 0.0;
                let mut origin = String::new();
                for pair in dep.expect("Failed to get table").sequence_values::<Value>() {
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
                            version = num_value;
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
            proj_version: proj_version.unwrap_or_else(|| panic!()),
            dependencies: DepManager::new(dependencies),
            proj_type,
        }
    }
}
