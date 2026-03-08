/// Adaptive Pulse and Density-Aware Scanning

pub struct PulseConfig {
    pub base_scan_interval_ms: u64,
    pub max_scan_interval_ms: u64,
}

impl Default for PulseConfig {
    fn default() -> Self {
        Self {
            base_scan_interval_ms: 200, // Aggressive baseline
            max_scan_interval_ms: 2000, // In dense networks
        }
    }
}

pub struct PulseManager {
    config: PulseConfig,
    active_peer_count: usize,
}

impl PulseManager {
    pub fn new() -> Self {
        Self {
            config: PulseConfig::default(),
            active_peer_count: 0,
        }
    }

    /// Update the number of peers currently seen in the BLE/AWDL aura
    pub fn update_density(&mut self, count: usize) {
        self.active_peer_count = count;
    }

    /// Calculate dynamic scan interval based on node density (Adaptive Pulse 1.0.2)
    pub fn get_scan_interval(&self) -> u64 {
        if self.active_peer_count > 15 {
            // High density (Office swarm) -> Relax scanning to save everybody's battery
            self.config.max_scan_interval_ms
        } else {
            // Low density -> Aggressive scanning to find a bridge
            self.config.base_scan_interval_ms
        }
    }
}
