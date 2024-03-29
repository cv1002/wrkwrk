#![allow(dead_code)]
#![allow(non_upper_case_globals)]
// Standard Mods
use std::sync::Arc;
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
use summary::SummaryUnit;

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
    #[arg(
        short,
        long,
        id = "ConnectionsAmount",
        help = "Connections to keep open"
    )]
    pub connections: usize,

    #[arg(short, long, id = "Seconds", help = "Duration of test")]
    pub duration: u64,

    #[arg(short, long, id = "ThreadsAmount", help = "Number of threads to use")]
    pub threads: usize,

    #[arg(long, help = "Use http1.0")]
    pub http10: bool,

    #[arg(long, help = "Use http1.1")]
    pub http11: bool,

    #[arg(long, help = "Use http2")]
    pub http2: bool,

    #[arg(long, help = "Use http3")]
    pub http3: bool,

    #[arg(long)]
    pub url: Option<String>,

    #[arg(short, long, id = "ScriptPath", help = "Load Lua script file")]
    pub script: Option<String>,

    #[arg(short = 'H', long, id = "AddAHeader", help = "Add header to request")]
    pub header: Option<Vec<String>>,

    #[arg(long, id = "TimeOut", help = "Socket/request timeout")]
    pub timeout: Option<u32>,
}

fn procedure(args: Arc<CommandLineArgs>) {
    let end_time = Instant::now() + Duration::from_secs(args.duration);
    // Send messages to server
    let handler = {
        // Sharing some datastructures
        let args = args.clone();
        // Create tokio runtime for later use
        let runtime = {
            if args.threads == 1 {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
            } else {
                tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(args.threads)
                    .enable_all()
                    .build()
                    .unwrap()
            }
        };

        // Each connection create a client, and a coroutine
        let worker = async {
            let clients = (0..args.connections)
                .map(|_| {
                    Client::new(WrkLuaVM::new(args.as_ref()).unwrap())
                        .unwrap()
                        .client_loop(&runtime, args.clone(), end_time)
                })
                .collect::<Vec<_>>();
            let mut summaryunits = Vec::with_capacity(clients.len());
            for client in clients {
                summaryunits.push(client.await.unwrap());
            }
            summaryunits
        };
        runtime.block_on(worker)
    };
    // Collect summaryunit
    let result = handler
        .into_iter()
        .fold(SummaryUnit::new(), SummaryUnit::merge);
    // Print some message
    println!(
        "Running {}s test; {} threads and {} connections; latency avg {:.3} us; total requests {}; avg requests {}/s; max latency {}us",
        args.duration,
        args.threads,
        args.connections,
        result.avg_latency(),
        result.total_request(),
        result.total_request() / args.duration,
        result.max_latency(),
    );
    // Release done
    let _ = WrkLuaVM::new(args.as_ref())
        .unwrap()
        .done(result.total_latency(), result.total_request());
}

fn main() {
    procedure(Arc::new(CommandLineArgs::parse()));
}
