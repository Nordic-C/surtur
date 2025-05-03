use std::path::PathBuf;

use mlua::Lua;

pub struct ScriptManager {
    pre_scripts: Vec<PathBuf>,
    post_scripts: Vec<PathBuf>,
    lua_ctx: Lua,
}

impl ScriptManager {
    pub fn new(pre_scripts: Vec<PathBuf>, post_scripts: Vec<PathBuf>) -> Self {
        Self {
            pre_scripts,
            post_scripts,
            lua_ctx: Lua::new(),
        }
    }

    pub fn pre_exec(&self) -> mlua::Result<()> {
        for script in &self.pre_scripts {
            self.lua_ctx.load(script.as_path()).exec()?;
        }
        Ok(())
    }

    pub fn post_exec(&self) -> mlua::Result<()> {
        for script in &self.post_scripts {
            self.lua_ctx.load(script.as_path()).exec()?;
        }
        Ok(())
    }
}
