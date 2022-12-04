// Standard Libs
use std::io::Read;
// External Libs
use mlua::Lua;
// Internal Mods
use crate::CommandLineArgs;

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
    pub fn get_vm(&self) -> &Lua {
        &self.lua
    }
    pub fn setup(&self) -> Result<(), mlua::Error> {
        self.lua.load("setup()").exec()?;
        Ok(())
    }
}
