use std::sync::Arc;

use tokio::time::Instant;

use crate::{lua::WrkLuaVM, CommandLineArgs};

pub struct Client {
    pub lua: Arc<WrkLuaVM>,
}
unsafe impl Send for Client {}
unsafe impl Sync for Client {}

impl Client {
    pub fn new(lua: Arc<WrkLuaVM>) -> Self {
        Self { lua }
    }
    pub async fn make_request(&mut self) {

    }
    pub async fn client_loop(self, args: Arc<CommandLineArgs>, end_time: Instant) {
        loop {
            match reqwest::get(&args.url).await {
                Err(_) => {}
                Ok(_) => {}
            };
            // At end time we end the procedure
            if Instant::now() >= end_time {
                break;
            }
        }
    }
}
