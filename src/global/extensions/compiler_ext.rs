use std::{env, fs, path::PathBuf};

use mlua::{Function, Lua};

use crate::{
    global::SURTUR_HOME,
    util::lua_utils::{LuaFunctionInfo, Values},
};

use super::Extension;

pub type CompilerCommand = String;

pub struct CompilerExt {}

impl<'f> Extension<'f, CompilerCommand, 2> for CompilerExt {
    fn extension_name(&self) -> &str {
        "compilers"
    }

    fn functions(&self) -> [LuaFunctionInfo; 2] {
        [
            LuaFunctionInfo::new("CompilerCommandBin", 6, Some(Values::String)),
            LuaFunctionInfo::new("CompilerCommandLib", 6, Some(Values::String)),
        ]
    }

    fn load_global(&self) -> anyhow::Result<[CompilerCommand; 2]> {
        let home_path = PathBuf::from(env::var(SURTUR_HOME)?);
        let path = home_path.join("extensions").join(self.extension_name());
        let files = fs::read_dir(path)?;
        for file in files {
            let lua = Lua::new();
            let content = fs::read_to_string(file?.path())?;
            lua.load(content).exec()?;
            let functions = self.functions();
            for function in functions {
                let func: Function = lua.globals().get(function.name)?;
                dbg!(func);
            }
        }
        todo!()
    }

    fn load_local(&self) -> anyhow::Result<[CompilerCommand; 2]> {
        todo!()
    }
}
