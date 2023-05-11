use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

use crate::{util::lazy::Lazy, LazyNew};

static REQUEST_COUNT: Lazy<AtomicU64> = LazyNew!(AtomicU64::new(0));
static TOTAL_LATENCY: Lazy<AtomicU64> = LazyNew!(AtomicU64::new(0));
static MAX_LATENCY: Lazy<AtomicU64> = LazyNew!(AtomicU64::new(0));

pub fn count_request() {
    REQUEST_COUNT.fetch_add(1, SeqCst);
}

pub fn add_latency(latency: u64) {
    MAX_LATENCY.fetch_max(latency, SeqCst);
    TOTAL_LATENCY.fetch_add(latency, SeqCst);
}

pub fn avg_latency() -> f64 {
    TOTAL_LATENCY.load(SeqCst) as f64 / REQUEST_COUNT.load(SeqCst) as f64
}

pub fn total_request() -> u64 {
    REQUEST_COUNT.load(SeqCst)
}

pub fn max_latency() -> u64 {
    MAX_LATENCY.load(SeqCst)
}
