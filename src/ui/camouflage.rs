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
    pub fn trigger_matryoshka(password_provided: bool) -> Result<(), &'static str> {
        if password_provided {
            // In a real scenario, this would check against the KDF offline hash.
            UI_CAT_DARK_MODE.store(true, Ordering::SeqCst);
            println!("[CAMOUFLAGE] Matryoshka layers shed. Cat_Dark Enterprise UI activated.");
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
