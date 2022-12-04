// Standard Libs
use std::io::Read;
// External Libs
use mlua::{Lua, LuaSerdeExt};
use serde::{Deserialize, Serialize};

use crate::CommandLineArgs;

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

pub struct WrkLuaVM {
    pub lua: Lua,

    __private: (),
}
impl WrkLuaVM {
    pub fn new(lua: Lua) -> Result<Self, mlua::Error> {
        let object = Self { lua, __private: () };
        object.setup()?;
        Ok(object)
    }
    pub fn get_wrk(&self, args: &CommandLineArgs) -> Result<Wrk, mlua::Error> {
        self.lua.load(include_str!("wrk.lua")).exec()?;
        if let Some(path) = args.script.as_deref() {
            // Open file
            let mut file = std::fs::File::open(path)?;
            // Read script from path
            let script = {
                let mut script = Vec::new();
                let _ = file.read_to_end(&mut script)?;
                script
            };
            self.lua.load(&script).exec()?;
        }
        let wrk: Wrk = {
            let wrk = self.lua.globals().get("wrk")?;
            self.lua.from_value(wrk)?
        };
        Ok(wrk)
    }
    pub fn setup(&self) -> Result<(), mlua::Error> {
        self.lua.load("setup()").exec()?;
        Ok(())
    }
}
