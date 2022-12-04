// Standard Mods
use std::collections::HashMap;
// External Mods
use mlua::{Function, LuaSerdeExt, Lua};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HttpRequest {
    pub host: Option<String>,
    pub port: Option<u32>,
    pub method: Option<String>,
    pub url: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<u32>,
    pub version: Option<String>,
}

impl HttpRequest {
    pub fn get_request(lua: &Lua) -> Result<HttpRequest, mlua::Error> {
        let request_fn: Function = lua.globals().get("request")?;
        lua.from_value(request_fn.call(())?)
    }
}
