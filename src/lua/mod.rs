// Standard Libs
use std::collections::HashMap;
// External Libs
use mlua::{Function, Lua, LuaSerdeExt};
// Internal Mods
use crate::CommandLineArgs;

pub struct WrkLuaVM {
    lua: Lua,
}
// Public fns
impl WrkLuaVM {
    pub fn new(args: &CommandLineArgs) -> Result<Self, mlua::Error> {
        // Build object
        let object = Self {
            lua: mlua::Lua::new(),
        };
        // Load wrk scripts and do setup.
        object.lua.load(include_str!("wrk.lua")).exec()?;
        object.setup()?;
        object.init(args)?;

        Ok(object)
    }
    pub fn get_vm(&self) -> &Lua {
        &self.lua
    }
    pub fn delay(&self) -> Result<(), mlua::Error> {
        let delay: Function = self.lua.globals().get("delay")?;
        delay.call(())
    }
    pub fn response(
        &self,
        status: u16,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    ) -> Result<(), mlua::Error> {
        let response: Function = self.lua.globals().get("response")?;
        response.call((status, headers, body))
    }
    pub fn done(&self, _summary: (), _latency: (), _requests: ()) {
        todo!()
    }
    fn setup(&self) -> Result<(), mlua::Error> {
        self.lua.load("wrk.setup()").exec()
    }
    fn init(&self, args: &CommandLineArgs) -> Result<(), mlua::Error> {
        // If commandline arguments have script, then run this script file
        let _ = args
            .script
            .as_deref()
            .map(|script| std::fs::read_to_string(script).unwrap())
            .map(|script| self.lua.load(&script).exec())
            .transpose()?;
        // Call init function
        let init: Function = self.lua.globals().get("init")?;
        init.call(self.lua.to_value(args))
    }
}
