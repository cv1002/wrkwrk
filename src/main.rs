#![allow(non_upper_case_globals)]
// Internal Mods
mod client;
mod lua;
use lua::WrkLuaVM;
// Standard Libs
use std::sync::Arc;
// External Libs
use clap::{command, Parser};
use client::Client;
use tokio::time::{Duration, Instant};

const about: &str = r#"
wrk is a modern HTTP benchmarking tool capable of generating significant load when run on a single multi-core CPU.
It combines a multithreaded design with scalable event notification systems such as epoll and kqueue.
"#;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = about
)]
pub struct CommandLineArgs {
    #[arg(long)]
    pub url: String,

    #[arg(
        short,
        long,
        id = "ConnectionsAmount",
        help = "Connections to keep open"
    )]
    pub connections: u32,

    #[arg(short, long, id = "Seconds", help = "Duration of test")]
    pub duration: u32,

    #[arg(short, long, id = "ThreadsAmount", help = "Number of threads to use")]
    pub threads: u32,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "Print latency statistics"
    )]
    pub latency: bool,

    #[arg(long, help = "Use http1.0")]
    pub http10: bool,

    #[arg(long, help = "Use http1.1")]
    pub http11: bool,

    #[arg(long, help = "Use http2")]
    pub http2: bool,

    #[arg(long, help = "Use http3")]
    pub http3: bool,

    #[arg(short, long, id = "ScriptPath", help = "Load Lua script file")]
    pub script: Option<String>,

    #[arg(short = 'H', long, id = "AddAHeader", help = "Add header to request")]
    pub header: Option<Vec<String>>,

    #[arg(long, id = "TimeOut", help = "Socket/request timeout")]
    pub timeout: Option<u32>,
}

/// TODO DISPLAY RESULT MESSAGE
fn display_result() {}

fn procedure(args: Arc<CommandLineArgs>) {
    let end_time = Instant::now() + Duration::from_secs(args.duration.into());
    // Send messages to server
    for _ in 0..args.threads {
        std::thread::spawn({
            // Sharing some datastructures
            let args = args.clone();
            let end_time = end_time.clone();
            // Main client loop
            move || {
                // Create tokio runtime for later use
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                // Each thread should create a lua virtual machine
                let lua_vm = Arc::new(WrkLuaVM::new(args.as_ref()).unwrap());
                // Each connection create a coroutine
                runtime.block_on(async {
                    for _ in 0..(args.connections / args.threads) {
                        runtime.spawn(
                            Client::new(lua_vm.clone())
                                .unwrap()
                                .client_loop(args.clone(), end_time.clone()),
                        );
                    }
                });
            }
        });
    }

    display_result();
}

fn main() {
    procedure(Arc::new(CommandLineArgs::parse()));
}
