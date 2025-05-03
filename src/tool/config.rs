use std::{
    collections::HashSet,
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
/// Handling of the project's lua config file.
/// It includes the lua parser and all information
/// related to the project's configuration
use mlua::{Lua, Table, Value};

use crate::util::{files::FileHandler, DEFAULT_COMPILER};

use super::{
    compiler::Standard,
    deps::{DepManager, Dependency},
    scripts::ScriptManager,
};

// TODO: Seperate tables from rest of the struct so it represents the actual config file
pub struct Config {
    pub name: String,
    pub props: Properties,
    pub deps: DepManager,
    pub entry: PathBuf,
    pub excluded: HashSet<PathBuf>,
    pub scripts: Option<ScriptManager>,
    pub libraries: HashSet<String>,
}

pub struct Properties {
    pub c_std: Standard,
    pub proj_version: String,
    pub proj_type: ProjType,
    pub compiler: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ProjType {
    Lib,
    Bin,
}

impl ProjType {
    pub fn from_str(c_std: &str) -> Option<ProjType> {
        match c_std {
            "lib" => Some(ProjType::Lib),
            "bin" => Some(ProjType::Bin),
            _ => None,
        }
    }
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
    pub fn parse(root_dir: &Path, file: FileHandler) -> anyhow::Result<Self> {
        let mut dependencies = HashSet::new();
        let mut libraries = HashSet::new();
        let mut excluded: HashSet<PathBuf> = HashSet::new();

        let lua = Lua::new();
        //let function = lua.create_function(builtins::run_script).expect("Failed to create function");

        //lua.globals().set("run_cmd", function).expect("Failed to set global function: \"run_cmd\"");

        lua.load(&file.file_content)
            .exec()
            .expect("Failed to load context");

        let name: String = lua
            .globals()
            .get("Name")
            .context("Failed to get name property even though it is required")?;

        // properties
        let props_table: Table = lua
            .globals()
            .get("Props")
            .expect("Failed to get properties");

        // dependencies
        let dep_table: Option<Table> = lua.globals().get("Dependencies").ok();

        let scripts_table: Option<Table> = lua.globals().get("Scripts").ok();

        let libraries_table: Option<Table> = lua.globals().get("Libraries").ok();

        let excluded_table: Option<Table> = lua.globals().get("Exclude").ok();

        let mut props = Properties {
            c_std: Standard::C23,
            proj_version: String::new(),
            proj_type: ProjType::Bin,
            compiler: String::from(DEFAULT_COMPILER),
        };

        for pair in props_table.pairs::<String, String>() {
            let (key, val) = pair.expect("Failed to get pair");
            match key.to_lowercase().as_str() {
                "std" => props.c_std = Standard::from_str(&val)
                    .context(format!("`{}` is not a valid value for the projects C Standard", val))?,
                "version" => props.proj_version = val,
                "compiler" => props.compiler = val,
                "type" => props.proj_type = ProjType::from_str(&val)
                    .context(format!("`{}` is not a valid value for the projects type. Valid types are: `lib` and `bin`", val))?,
                key => bail!("invalid version entry: {}", key),
            }
        }

        if let Some(table) = excluded_table {
            for elem in table.sequence_values::<String>().flatten() {
                excluded.insert(root_dir.join("src").join(elem));
            }
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

        let entry: String = lua.globals().get("Entry").unwrap_or(
            match props.proj_type {
                ProjType::Lib => "lib.c",
                ProjType::Bin => "main.c",
            }
            .into(),
        );

        let mut pre_scripts = Vec::new();
        let mut post_scripts = Vec::new();

        if let Some(table) = scripts_table {
            for (key, val) in table.pairs::<String, Table>().flatten() {
                match key.as_str() {
                    "pre" => {
                        pre_scripts = val
                            .sequence_values::<String>()
                            .map(|val| PathBuf::from(val.unwrap()))
                            .collect()
                    }
                    "post" => {
                        post_scripts = val
                            .sequence_values::<String>()
                            .map(|val| PathBuf::from(val.unwrap()))
                            .collect()
                    }
                    key => bail!("Found invalid key: {key}"),
                }
            }
        }

        let scripts = if pre_scripts.is_empty() && post_scripts.is_empty() {
            None
        } else {
            Some(ScriptManager::new(pre_scripts, post_scripts))
        };

        if let Some(table) = libraries_table {
            for lib in table.sequence_values::<String>() {
                let lib = lib.context("Failed to get library")?;
                libraries.insert(lib);
            }
        }

        Ok(Self {
            name,
            props,
            deps: DepManager::new(dependencies),
            entry: entry.into(),
            excluded,
            scripts,
            libraries,
        })
    }
}
