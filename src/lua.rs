// Standard Libs
use std::io::Read;
// External Libs
use mlua::{LuaSerdeExt, Lua};
use serde::{Deserialize, Serialize};

use crate::args;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Wrk {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub port: Option<u32>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub headers: Option<String>,
    pub body: Option<String>,
    pub thread: Option<()>,
}

pub fn get_wrk(lua: &Lua) -> Wrk {
    lua.load(include_str!("wrk.lua")).exec().unwrap();
    if let Some(path) = args.script.as_deref() {
        // Open file
        let mut file = std::fs::File::open(path).unwrap();
        // Read script from path
        let script = {
            let mut script = Vec::new();
            let _ = file.read_to_end(&mut script).unwrap();
            script
        };
        lua.load(&script).exec().unwrap();
    }
    let wrk: Wrk = {
        let wrk = lua.globals().get("wrk").unwrap();
        lua.from_value(wrk).unwrap()
    };
    return wrk;
}
