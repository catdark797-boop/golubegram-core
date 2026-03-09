use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;

lazy_static! {
    /// Global UI state. False = Golubegram (Innocent), True = Cat_Dark (Active)
    pub static ref UI_CAT_DARK_MODE: AtomicBool = AtomicBool::new(false);
}

/// The Matryoshka UI State Manager
pub struct CamouflageState;

impl CamouflageState {
    pub fn is_active() -> bool {
        UI_CAT_DARK_MODE.load(Ordering::Relaxed)
    }

    /// Triggered by the specific gesture + valid password hash
    /// Hash A (Real) unlocks network. Hash B (Duress) zeroizes keys and opens a dummy profile.
    pub fn trigger_matryoshka(hash_provided: &str) -> Result<(), &'static str> {
        let real_hash = "hashed_real_kdf_key"; // Mock: In reality, from secure storage
        let duress_hash = "hashed_panic_kdf_key"; // Mock Vault Backup Key

        if hash_provided == real_hash {
            UI_CAT_DARK_MODE.store(true, Ordering::SeqCst);
            println!("[CAMOUFLAGE] Matryoshka layers shed. Cat_Dark Enterprise UI activated.");
            Ok(())
        } else if hash_provided == duress_hash {
            println!("[DURESS TRIGGERED] PANIC HASH ENTERED. INITIATING SCORCHED EARTH PROTOCOL.");
            // 1. Wipe real SQLite keys
            crate::crypto::hidden::panic_lock(); // Assuming this zeroizes keys 
            
            // 2. Open fake empty UI
            UI_CAT_DARK_MODE.store(true, Ordering::SeqCst);
            println!("[CAMOUFLAGE] Dummy Profile Loaded. Token Wallet: 0. Chats: Empty.");
            Ok(())
        } else {
            Err("Invalid Matryoshka trigger.")
        }
    }

    /// Panic lock: Revert UI to Golubegram immediately
    pub fn revert_to_innocent() {
        UI_CAT_DARK_MODE.store(false, Ordering::SeqCst);
        println!("[CAMOUFLAGE] Reverted to Golubegram Innocent UI.");
    }
}
