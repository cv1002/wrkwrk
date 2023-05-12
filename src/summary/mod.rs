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
