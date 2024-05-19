use std::{collections::HashSet, fmt::Display, path::PathBuf};

use anyhow::{bail, Context};
/// Handling of the project's lua config file.
/// It includes the lua parser and all information
/// related to the project's configuration
use mlua::{Lua, Table, Value};

use crate::util::{files::FileHandler, DEFAULT_COMPILER};

use super::{
    compiler::{Standard, STANDARDS},
    deps::{DepManager, Dependency},
    scripts::ScriptManager,
};

pub struct Config {
    pub compiler: String,
    pub c_std: Standard,
    pub proj_version: String,
    pub proj_type: ProjType,
    pub deps: DepManager,
    pub entry: PathBuf,
    pub excluded: HashSet<PathBuf>,
    pub scripts: Option<ScriptManager>,
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
    pub fn parse(root_dir: &PathBuf, file: FileHandler) -> anyhow::Result<Self> {
        let mut dependencies = HashSet::new();
        let mut c_std_str = String::from("c17");
        let mut proj_version = None;
        let mut proj_type = ProjType::Lib;
        let mut compiler = String::from(DEFAULT_COMPILER);
        let mut excluded: HashSet<PathBuf> = HashSet::new();

        let mut c_std: Option<Standard> = None;

        let lua = Lua::new();
        //let function = lua.create_function(builtins::run_script).expect("Failed to create function");

        //lua.globals().set("run_cmd", function).expect("Failed to set global function: \"run_cmd\"");

        lua.load(&file.file_content)
            .exec()
            .expect("Failed to load context");

        // properties
        let props_table: Table = lua
            .globals()
            .get("Props")
            .expect("Failed to get properties");

        // dependencies
        let dep_table: Option<Table> = lua.globals().get("Dependencies").ok();

        let scripts_table: Option<Table> = lua.globals().get("Scripts").ok();

        let entry: String = lua.globals().get("Entry").unwrap_or(
            match proj_type {
                ProjType::Lib => "lib.c",
                ProjType::Bin => "main.c",
            }
            .into(),
        );

        let excluded_table: Option<Table> = lua.globals().get("Exclude").ok();

        for pair in props_table.pairs::<String, String>() {
            let (key, val) = pair.expect("Failed to get pair");
            match key.to_lowercase().as_str() {
                "std" => c_std_str = val,
                "version" => proj_version = Some(val),
                "compiler" => compiler = val,
                "type" => {
                    proj_type = match val.as_str() {
                        "lib" => ProjType::Lib,
                        "bin" => ProjType::Bin,
                        _ => bail!("`{}` is not a valid value for the projects type. Valid types are: `lib` and `bin`", val),
                    }
                }
                key => bail!("invalid version entry: {}", key),
            }
        }

        match excluded_table {
            Some(table) => {
                for elem in table.sequence_values::<String>() {
                    match elem {
                        Ok(elem) => {
                            excluded.insert(root_dir.join("src").join(elem));
                        }
                        Err(_) => (),
                    }
                }
            }
            None => (),
        }

        // Iterating over dependencies
        if let Some(deps) = dep_table {
            for dep in deps.sequence_values::<Table>() {
                let mut version = 0.0;
                let mut origin = String::new();
                let table = dep.context("Failed to get dependency table")?;
                for pair in table.sequence_values::<Value>() {
                    match pair.context("Failed to get dependency pair")? {
                        Value::Integer(int_value) => {
                            version = int_value as f64;
                        }
                        Value::String(string_value) => {
                            origin = string_value.to_string_lossy().to_string();
                        }
                        Value::Number(num_value) => {
                            version = num_value;
                        }
                        val => {
                            bail!("Invalid value in dependency table, value: {val:?}")
                        }
                    }
                }
                let dependency = Dependency::new(&origin, &version.to_string());
                dependencies.insert(dependency);
            }
        }

        // version selection
        for std in STANDARDS {
            if c_std_str == std.to_string() {
                c_std = Some(std);
                break;
            }
        }

        let mut pre_scripts = Vec::new();
        let mut post_scripts = Vec::new();

        if let Some(table) = scripts_table {
            for scripts in table.pairs::<String, Table>() {
                if let Ok((key, val)) = scripts {
                    match key.as_str() {
                        "pre" => {
                            pre_scripts = val
                                .sequence_values::<String>()
                                .into_iter()
                                .map(|val| PathBuf::from(val.unwrap()))
                                .collect()
                        }
                        "post" => {
                            post_scripts = val
                                .sequence_values::<String>()
                                .into_iter()
                                .map(|val| PathBuf::from(val.unwrap()))
                                .collect()
                        }
                        key => bail!("Found invalid key: {key}"),
                    }
                }
            }
        }

        let scripts = if pre_scripts.is_empty() && post_scripts.is_empty() {
            None
        } else {
            Some(ScriptManager::new(pre_scripts, post_scripts))
        };

        Ok(Self {
            compiler,
            c_std: match c_std {
                Some(std) => std,
                None => bail!("Invalid C Standard: {:?}", c_std_str),
            },
            proj_version: proj_version.context("Failed to get project version")?,
            deps: DepManager::new(dependencies),
            proj_type,
            entry: entry.into(),
            excluded,
            scripts,
        })
    }
}
