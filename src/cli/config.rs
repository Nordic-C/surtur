use std::{collections::HashSet, fmt::Display, path::PathBuf};

/// Handling of the project's lua config file.
/// It includes the lua parser and all information
/// related to the project's configuration
use clutils::files::FileHandler;
use rlua::{Lua, Result, Table, Value};
use strum::IntoEnumIterator;

use crate::util::DEFAULT_COMPILER;

use super::{
    compiler::Standard,
    deps::{DepManager, Dependency},
};

pub struct Config {
    pub compiler: String,
    pub c_std: Standard,
    pub proj_version: String,
    pub proj_type: ProjType,
    pub deps: DepManager,
    pub entry: PathBuf,
    pub excluded: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
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

impl Config {
    pub fn from(file: FileHandler) -> Self {
        let mut dependencies = HashSet::new();
        let mut c_std_str = String::from("c17");
        let mut proj_version = None;
        let mut proj_type = ProjType::Lib;
        let mut compiler = String::from(DEFAULT_COMPILER);
        let entry: String;
        let mut excluded_vec: Vec<PathBuf> = vec![];

        let stds: Vec<Standard> = Standard::iter().collect();
        let mut c_std: Option<Standard> = None;

        let lua = Lua::new();
        //let function = lua.create_function(builtins::run_script).expect("Failed to create function");

        //lua.globals().set("run_cmd", function).expect("Failed to set global function: \"run_cmd\"");

        lua.load(&file.content)
            .exec()
            .expect("Failed to load context");

        // dependencies
        let dep_table: Table = lua
            .globals()
            .get("Dependencies")
            .expect("Failed to get dependencies");

        // properties
        let props_table: Table = lua
            .globals()
            .get("Props")
            .expect("Failed to get properties");

        entry = lua.globals().get("Entry").unwrap_or(match proj_type {
            ProjType::Lib => "lib.c",
            ProjType::Bin => "main.c",
        }.into());

        let excluded: Result<Table> = lua.globals().get("Exclude");

        props_table
                .pairs::<String, String>()
                .for_each(|pair| {
                    let (key, val) = pair.expect("Failed to get pair");
                    match key.to_lowercase().as_str() {
                        "std" => c_std_str = val,
                        "version" => proj_version = Some(val),
                        "compiler" => compiler = val,
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

        match excluded {
            Ok(table) => {
                for elem in table.pairs::<u64, String>() {
                    match elem {
                        Ok(elem) => excluded_vec.push(elem.1.into()),
                        Err(_) => (),
                    }
                }
            }
            Err(_) => (),
        }

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
            let dependency = Dependency::new(&origin, &version.to_string());
            dependencies.insert(dependency);
        }

        // version selection
        for std in stds {
            if c_std_str == std.to_string() {
                c_std = Some(std);
                break;
            }
        }

        Self {
            compiler,
            c_std: match c_std {
                Some(std) => std,
                None => panic!("Invalid C Standard: {:?}", c_std_str),
            },
            proj_version: proj_version.unwrap_or_else(|| panic!()),
            deps: DepManager::new(dependencies),
            proj_type,
            entry: entry.into(),
            excluded: excluded_vec,
        }
    }
}
