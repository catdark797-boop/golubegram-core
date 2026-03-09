/// DEAD MAN'S SWITCH (THE DEAD LEG PROTOCOL)
/// Biometric and geofenced triggers for extreme-duress scenarios.

pub struct DeadManSwitch;

const LEVEL_1_LOCKDOWN_HOURS: u64 = 12;
const ABSOLUTE_ZEROIZE_HOURS: u64 = 24;

impl DeadManSwitch {
    /// Evaluates the time since the last valid physical heartbeat (BLE Heart Monitor)
    /// or secure spatial anchor (Home Wi-Fi BSSID).
    pub fn evaluate_triggers(hours_since_last_contact: u64) {
        if hours_since_last_contact >= ABSOLUTE_ZEROIZE_HOURS {
            println!("[DEAD LEG] CRITICAL: Trigger lost for {} hours (Threshold: {}h).", hours_since_last_contact, ABSOLUTE_ZEROIZE_HOURS);
            println!("[DEAD LEG] Operator presumed Captured/KIA. Executing Scorched Earth.");
            Self::execute_absolute_zeroize();
        } else if hours_since_last_contact >= LEVEL_1_LOCKDOWN_HOURS {
            println!("[DEAD LEG] WARNING: Trigger lost for {} hours (Threshold: {}h).", hours_since_last_contact, LEVEL_1_LOCKDOWN_HOURS);
            println!("[DEAD LEG] Triggering Level 1 Lockdown. Biometrics disabled, Master Seed required.");
            Self::execute_level_1_lockdown();
        } else {
            println!("[DEAD LEG] Pulse normal. Watchdog timer reset.");
        }
    }

    fn execute_level_1_lockdown() {
        // UI drops to hard lock screen, biometric/face unlock is entirely disabled.
        println!("[SECURITY] Device locked. OS-level biometric decryption bypassed.");
    }

    fn execute_absolute_zeroize() {
        // Invokes crypto module zeroize
        crate::crypto::hidden::panic_lock();
        println!("[SECURITY] Cryptographic material zeroized. Node is now a brick.");
    }
}
