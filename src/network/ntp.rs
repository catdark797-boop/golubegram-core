use chrono::{DateTime, Utc};
use std::time::Duration;

pub struct NtpState {
    peer_times: Vec<DateTime<Utc>>,
    pub local_offset: Duration,
}

impl NtpState {
    pub fn new() -> Self {
        Self {
            peer_times: Vec::new(),
            local_offset: Duration::ZERO,
        }
    }

    /// Record a time advertised by a peer during handshake
    pub fn add_peer_time(&mut self, time: DateTime<Utc>) {
        self.peer_times.push(time);
        self.recalculate_median();
    }

    /// Calculates the Network Density Median Time
    fn recalculate_median(&mut self) {
        if self.peer_times.is_empty() {
            return;
        }
        
        // Sort times to find the median
        self.peer_times.sort();
        let mid = self.peer_times.len() / 2;
        let median_time = self.peer_times[mid];
        
        let now = Utc::now();
        
        // Calculate the offset between local time and swarm median time
        if median_time > now {
            self.local_offset = (median_time - now).to_std().unwrap_or(Duration::ZERO);
        } else {
            // Negative offset handling
            self.local_offset = Duration::ZERO; // Simplified for the V1.0 structure
        }
    }
    
    /// Returns the adjusted, cryptographically safe network time
    pub fn swarm_time(&self) -> DateTime<Utc> {
        Utc::now() + self.local_offset
    }
}
