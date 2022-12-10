// Standard Mods
use std::collections::HashMap;
// External Mods
use mlua::{Function, LuaSerdeExt, Lua};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HttpRequest {
    pub scheme: String,
    pub host: String,
    pub port: u32,
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub timeout: u32,
    pub version: String,
    // Fields that could be none
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn get_request(lua: &Lua) -> Result<HttpRequest, mlua::Error> {
        let request_fn: Function = lua.globals().get("request")?;
        lua.from_value(request_fn.call(())?)
    }
}
