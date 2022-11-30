#![allow(non_upper_case_globals)]
// Internal Mods
mod lua;
mod client;
// Standard Libs
use std::sync::atomic::{AtomicU32, Ordering};
// External Libs
use clap::{command, Parser};
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
    threads: usize,

    #[arg(short, long)]
    latency: bool,

    #[arg(long)]
    script: Option<String>,

    #[arg(short = 'H', long)]
    header: Option<String>,

    #[arg(long)]
    timeout: Option<u32>,
}

lazy_static::lazy_static!(
    /// Command line arguments
    static ref args: CommandLineArgs = CommandLineArgs::parse();
    /// Connections count: keep connections count just equals ARGS.connections
    static ref connections_count: AtomicU32 = AtomicU32::new(0);
    /// Success count
    static ref success_count: AtomicU32 = AtomicU32::new(0);
    /// Failure count
    static ref failure_count: AtomicU32 = AtomicU32::new(0);
);

async fn client_get() -> Result<(), Box<dyn std::error::Error>> {
    let _ = reqwest::get(&args.url).await?;
    Ok(())
}

/// TODO DISPLAY RESULT MESSAGE
fn display_result() {}

// 1.一比一复刻WRK的特性
// 2.给WRK增加HTTP2的功能
fn main() {
    let wrk = lua::get_wrk();
    // Create tokio runtime for later use
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.threads)
        .enable_all()
        .build()
        .unwrap();
    let end_time = Instant::now() + Duration::from_secs(args.duration.into());
    // Send messages to server
    for _ in 0..args.connections {
        let end_time = end_time.clone();
        runtime.spawn(async move {
            loop {
                match client_get().await {
                    Err(_) => {
                        failure_count.fetch_add(1, Ordering::SeqCst);
                    }
                    Ok(_) => {
                        success_count.fetch_add(1, Ordering::SeqCst);
                    }
                };
                // At end time we end the procedure
                if Instant::now() >= end_time {
                    break;
                }
            }
        });
    }
    display_result();
}
