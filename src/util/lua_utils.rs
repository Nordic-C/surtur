#[derive(Debug, Clone, Copy)]
pub enum Values {
    Nil,
    Boolean,
    LightUserData,
    Integer,
    Number,
    String,
    Table,
    Function,
    Thread,
    UserData,
    Error,
}

pub struct LuaFunctionInfo {
    pub name: &'static str,
    pub args: usize,
    pub ret_type: Option<Values>,
}

impl LuaFunctionInfo {
    pub fn new(name: &'static str, args: usize, ret_type: Option<Values>) -> LuaFunctionInfo {
        LuaFunctionInfo {
            name,
            args,
            ret_type
        }
    }
}
