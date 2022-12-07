// Standard Libs
use std::{io::Read, collections::HashMap};
// External Libs
use mlua::{Lua, Function, LuaSerdeExt};
// Internal Mods
use crate::CommandLineArgs;

pub struct WrkLuaVM {
    lua: Lua,

    __private: (),
}
// Public fns
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
        object.init(args)?;
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
    pub fn get_vm(&self) -> &Lua {
        &self.lua
    }
    pub fn delay(&self) -> Result<(), mlua::Error> {
        let delay: Function = self.lua.globals().get("delay")?;
        delay.call(())
    }
    pub fn response(&self, status: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Result<(), mlua::Error> {
        let response: Function = self.lua.globals().get("response")?;
        response.call((status, headers, body))
    }
    pub fn done(&self, _summary: (), _latency: (), _requests: ()) {
        todo!()
    }
    fn setup(&self) -> Result<(), mlua::Error> {
        self.lua.load("setup()").exec()
    }
    fn init(&self, args: &CommandLineArgs) -> Result<(), mlua::Error> {
        let init: Function = self.lua.globals().get("init")?;
        init.call(self.lua.to_value(args))
    }
}
