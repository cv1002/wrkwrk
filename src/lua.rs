// Standard Libs
use std::{collections::HashMap, io::Read};
// External Libs
use mlua::{Function, Lua, LuaSerdeExt};
use serde::{Deserialize, Serialize};
// Internal Mods
use crate::CommandLineArgs;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Wrk {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub port: Option<u32>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub headers: Option<String>,
    pub body: Option<Vec<u8>>,
    pub thread: Option<()>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HttpRequest {
    pub host: Option<String>,
    pub port: Option<u32>,
    pub method: Option<String>,
    pub url: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<u32>,
}

pub struct WrkLuaVM {
    lua: Lua,

    __private: (),
}
impl WrkLuaVM {
    pub fn new(args: &CommandLineArgs) -> Result<Self, mlua::Error> {
        // Build object
        let object = Self {
            lua: mlua::Lua::new(),
            __private: (),
        };
        // Load wrk scripts and do setup.
        object.lua.load(include_str!("wrk.lua")).exec()?;
        object.setup()?;

        // If commandline arguments have script, then run this script file
        if let Some(path) = args.script.as_deref() {
            // Open file
            let mut file = std::fs::File::open(path)?;
            // Read script from path
            let script = {
                let mut script = Vec::new();
                let _ = file.read_to_end(&mut script)?;
                script
            };
            object.lua.load(&script).exec()?;
        }

        Ok(object)
    }
    pub fn get_wrk(&self) -> Result<Wrk, mlua::Error> {
        let wrk = self.lua.globals().get("wrk")?;
        self.lua.from_value(wrk)
    }
    pub fn get_request(&self) -> Result<HttpRequest, mlua::Error> {
        let request_fn: Function = self.lua.globals().get("request")?;
        self.lua.from_value(request_fn.call(())?)
    }
    pub fn setup(&self) -> Result<(), mlua::Error> {
        self.lua.load("setup()").exec()?;
        Ok(())
    }
}
