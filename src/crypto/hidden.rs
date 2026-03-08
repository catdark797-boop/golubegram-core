use zeroize::Zeroize;
use std::sync::Mutex;
use lazy_static::lazy_static;

/// The Shrodinger Protocol Context
/// Plausible Deniability Storage. Decrypted via offline KDF password logic.
#[derive(Default)]
pub struct ShrodingerContext {
    pub active_secret_key: [u8; 32],
    pub session_token: [u8; 64],
}

// Ensure the memory gets wiped securely when the struct is dropped
impl Drop for ShrodingerContext {
    fn drop(&mut self) {
        self.active_secret_key.zeroize();
        self.session_token.zeroize();
    }
}

lazy_static! {
    /// Hidden Layer exists solely in RAM and gets completely zeroed out upon lock.
    pub static ref HIDDEN_LAYER: Mutex<Option<ShrodingerContext>> = Mutex::new(None);
}

/// Simulated Unlock process via Hash derived from a master password 
pub fn unlock_hidden_layer(_password_hash: &[u8]) {
    let mut ctx = ShrodingerContext::default();
    
    // In a real implementation we expand the user hash using Argon2
    // Here we just mock the payload activation
    ctx.active_secret_key.copy_from_slice(&[0x42; 32]);
    ctx.session_token.copy_from_slice(&[0x77; 64]);
    
    let mut guard = HIDDEN_LAYER.lock().unwrap();
    *guard = Some(ctx);
    println!("[SHRODINGER] Hidden context unlocked in RAM.");
}

/// Emergency Nuke (OS Suspend / App Moved to Background / Lock Trigger)
pub fn panic_lock() {
    let mut guard = HIDDEN_LAYER.lock().unwrap();
    // take() removes the Some(ctx), causing it to drop and explicitly trigger `zeroize()`
    let _ = guard.take();
    println!("[SHRODINGER] Context ZEROIZED. Plausible Deniability Active.");
}
