/// Adaptive Duty Cycling Logic (Battery Life Preservation)
use std::sync::atomic::{AtomicU8, Ordering};

pub struct BatteryMonitor;

/// Global state representing the current reported capacity of the device OS battery (0-100%).
pub static OS_BATTERY_LEVEL: AtomicU8 = AtomicU8::new(100);

/// Power states for the Mesh Node.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodePowerState {
    ActiveTransit, // Full mesh participation
    LeechMode,     // RX Only, ignores 3rd party TX transit to save power
    NukeQuarantine,// Immediate power cycle / Burn Notice
}

impl BatteryMonitor {
    /// Determines the current networking posture based on battery life.
    pub fn evaluate_posture() -> NodePowerState {
        let current_lvl = OS_BATTERY_LEVEL.load(Ordering::Relaxed);
        
        if current_lvl < 15 {
            println!("[BATTERY WARN] Capacity < 15% ({}%). Switching to LEECH MODE (RX Only).", current_lvl);
            NodePowerState::LeechMode
        } else {
            NodePowerState::ActiveTransit
        }
    }

    /// Simulate OS Intent battery broadcast update (e.g. from Android via FFI)
    pub fn update_os_level(level: u8) {
        let clamped = level.min(100);
        OS_BATTERY_LEVEL.store(clamped, Ordering::Relaxed);
    }
}
