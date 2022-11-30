#![allow(non_upper_case_globals)]
// Internal Mods
mod client;
mod lua;
// Standard Libs
use std::sync::atomic::AtomicU32;
// External Libs
use clap::{command, Parser};
use client::Client;
use tokio::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CommandLineArgs {
    #[arg(long)]
    url: String,

    #[arg(short, long)]
    connections: u32,

    #[arg(short, long)]
    duration: u32,

    #[arg(short, long)]
    threads: u32,

    #[arg(short, long)]
    latency: bool,

    #[arg(long)]
    http2: bool,

    #[arg(short, long)]
    script: Option<String>,

    #[arg(short = 'H', long)]
    header: Option<String>,

    #[arg(long)]
    timeout: Option<u32>,
}

lazy_static::lazy_static! {
    /// Command line arguments
    static ref args: CommandLineArgs = CommandLineArgs::parse();
    /// Connections count: keep connections count just equals ARGS.connections
    static ref connections_count: AtomicU32 = AtomicU32::new(0);
    /// Success count
    static ref success_count: AtomicU32 = AtomicU32::new(0);
    /// Failure count
    static ref failure_count: AtomicU32 = AtomicU32::new(0);
}

/// TODO DISPLAY RESULT MESSAGE
fn display_result() {}

// 1.一比一复刻WRK的特性
// 2.给WRK增加HTTP2的功能
fn main() {
    // Create lua runtime
    let lua = mlua::Lua::new();
    let wrk = lua::get_wrk(&lua);

    let end_time = Instant::now() + Duration::from_secs(args.duration.into());
    // Send messages to server
    for _ in 0..args.threads {
        std::thread::spawn({
            let end_time = end_time.clone();
            move || {
                // Create tokio runtime for later use
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                for _ in 0..(args.connections / args.threads) {
                    let end_time = end_time.clone();
                    runtime.spawn(Client {}.client_loop(end_time));
                }
            }
        });
    }

    display_result();
}
