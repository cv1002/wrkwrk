use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

use crate::{util::lazy::Lazy, LazyNew};

static REQUEST_COUNT: Lazy<AtomicU64> = LazyNew!(AtomicU64::new(0));
static TOTAL_LATENCY: Lazy<AtomicU64> = LazyNew!(AtomicU64::new(0));
static MAX_LATENCY: Lazy<AtomicU64> = LazyNew!(AtomicU64::new(0));

pub struct SummaryUnit {
    request_count: u64,
    total_latency: u64,
    max_latency: u64,
}
impl SummaryUnit {
    pub fn new() -> Self {
        SummaryUnit {
            request_count: 0,
            total_latency: 0,
            max_latency: 0,
        }
    }
    pub fn count_request(&mut self) {
        self.request_count += 1;
    }
    pub fn add_latency(&mut self, latency: u64) {
        if self.max_latency < latency {
            self.max_latency = latency;
        }
        self.total_latency += latency;
    }
    pub fn avg_latency(&self) -> f64 {
        self.total_latency as f64 / self.request_count as f64
    }
    pub fn total_request(&self) -> u64 {
        self.request_count
    }
    pub fn max_latency(&self) -> u64 {
        self.max_latency
    }
    pub fn merge(self, other: Self) -> Self {
        SummaryUnit {
            request_count: self.request_count + other.request_count,
            total_latency: self.total_latency + other.total_latency,
            max_latency: self.max_latency.max(other.max_latency),
        }
    }
}

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
