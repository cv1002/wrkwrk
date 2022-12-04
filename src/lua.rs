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
        object.register_lookup();
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
impl WrkLuaVM {
    fn register_lookup(&self) -> Result<(), mlua::Error> {
        let script_wrk_lookup =
            self.lua
                .create_function(|_, (host, service): (String, String)| unsafe {
                    use libc::{addrinfo, c_char, SOCK_STREAM};
                    const PF_UNSPEC: i32 = 0;

                    let mut addrs: *mut libc::addrinfo = std::ptr::null_mut::<addrinfo>();
                    let hints = libc::addrinfo {
                        // Useless parameter
                        ai_flags: Default::default(),
                        ai_protocol: Default::default(),
                        ai_addrlen: Default::default(),
                        ai_canonname: std::ptr::null_mut(),
                        ai_addr: std::ptr::null_mut(),
                        ai_next: std::ptr::null_mut(),
                        // Useful parameter
                        ai_family: PF_UNSPEC,
                        ai_socktype: SOCK_STREAM,
                    };

                    let rc = libc::getaddrinfo(
                        host.as_ptr() as *const c_char,
                        service.as_ptr() as *const c_char,
                        &hints as *const addrinfo,
                        &mut addrs as *mut *mut addrinfo,
                    );

                    if rc != 0 {
                        let msg = libc::gai_strerror(rc);
                        eprint!(
                            "unable to resolve {}:{} {}\n",
                            host,
                            service,
                            std::ffi::CStr::from_ptr(msg as *const c_char)
                                .to_str()
                                .unwrap()
                        );
                        libc::exit(1);
                    }

                    libc::freeaddrinfo(addrs);
                    Ok(())
                })?;
        self.lua.globals().set("lookup", script_wrk_lookup)
    }
}
