pub mod compiler_ext;

use mlua::FromLua;

use crate::util::lua_utils::LuaFunctionInfo;

pub trait Extension<'f, T: FromLua, const FUNCTIONS: usize> {
    fn name(&self) -> &str;

    fn functions(&self) -> [LuaFunctionInfo; FUNCTIONS];

    fn load_global(&self) -> anyhow::Result<[T; FUNCTIONS]>;

    fn load_local(&self) -> anyhow::Result<[T; FUNCTIONS]>;
}
