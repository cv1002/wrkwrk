use std::sync::atomic::Ordering;

use tokio::time::Instant;

use crate::{args, failure_count, success_count};
pub struct Client {}

impl Client {
    pub async fn client_loop(self, end_time: Instant) {
        async fn client_get() -> Result<(), Box<dyn std::error::Error>> {
            let _ = reqwest::get(&args.url).await?;
            Ok(())
        }
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
    }
}
