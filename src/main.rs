#![allow(dead_code)]
#![allow(non_upper_case_globals)]
// Standard Mods
use std::{
    any::Any,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};
// External Mods
use clap::{command, Parser};
use client::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, Instant};
// Internal Mods
mod client;
mod lua;
mod summary;
mod util;
use lua::WrkLuaVM;

const about: &str = r#"
wrk is a modern HTTP benchmarking tool capable of generating significant load when run on a single multi-core CPU.
It combines a multithreaded design with scalable event notification systems such as epoll and kqueue.
"#;

#[derive(Parser, Debug, Serialize, Deserialize)]
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
    pub connections: usize,

    #[arg(short, long, id = "Seconds", help = "Duration of test")]
    pub duration: usize,

    #[arg(short, long, id = "ThreadsAmount", help = "Number of threads to use")]
    pub threads: usize,

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

fn procedure(args: Arc<CommandLineArgs>) -> Vec<Result<(), Box<dyn Any + Send>>> {
    let mut id = 0;
    let end_time = Instant::now() + Duration::from_secs(args.duration as u64);
    // Send messages to server
    let handler = |_| {
        std::thread::spawn({
            // Sharing some datastructures
            let args = args.clone();
            // Create tokio runtime for later use
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            // Main client loop
            move || {
                // Each thread should create a lua virtual machine
                let lua_vm = Arc::new(WrkLuaVM::new(args.as_ref()).unwrap());
                // Each connection create a coroutine
                runtime.block_on(async {
                    for _ in 0..(args.connections / args.threads) {
                        Client::new(id, lua_vm.clone())
                            .unwrap()
                            .client_loop(&runtime, args.clone(), end_time);
                        id += 1;
                    }
                });
            }
        })
    };
    let results = vec![(); args.threads as usize]
        .into_iter()
        .map(handler)
        .map(|handle| handle.join())
        .collect::<Vec<_>>();

    results
}

fn main() {
    let _ = procedure(Arc::new(CommandLineArgs::parse()));
}
