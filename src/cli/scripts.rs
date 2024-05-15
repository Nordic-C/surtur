use std::path::PathBuf;

use mlua::Lua;

pub struct ScriptManager {
    scripts: Vec<PathBuf>,
    lua_ctx: Lua,
}

impl ScriptManager {
    pub fn new(scripts: Vec<PathBuf>) -> Self {
        Self { scripts, lua_ctx: Lua::new() }
    }

    pub fn exec(&self) -> mlua::Result<()> {
        for script in &self.scripts {
            self.lua_ctx.load(script.as_path()).exec()?;
        }
        Ok(())
    }
}
