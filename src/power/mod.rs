pub mod pulse;
pub mod allocator;

/// Manages logical Duty Cycling and Battery Egoism
pub struct PowerMonitor {
    pub battery_level: u8,
    pub is_charging: bool,
    pub pulse_manager: pulse::PulseManager,
    pub transit_ram: allocator::RamAllocator,
}

impl PowerMonitor {
    pub fn new() -> Self {
        Self {
            battery_level: 100,
            is_charging: false,
            pulse_manager: pulse::PulseManager::new(),
            transit_ram: allocator::RamAllocator::new(32), // Strict 32 MB transit limit for low-end devices
        }
    }

    /// Returns true if device is in 'Egoist' mode (survival)
    pub fn is_egoist(&self) -> bool {
        self.battery_level <= 15 && !self.is_charging
    }

    /// Returns true if device is connected to power ('Super-Node' mode) -> Aggressive routing + OTA donor
    pub fn is_super_node(&self) -> bool {
        self.is_charging
    }
}
