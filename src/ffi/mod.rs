pub mod ios;
pub mod android;

/// Core C-FFI API endpoints exposed to Swift / Kotlin

#[no_mangle]
pub extern "C" fn golubegram_init() {
    crate::init_core();
}

#[no_mangle]
pub extern "C" fn golubegram_panic_lock() {
    crate::crypto::hidden::panic_lock();
}

#[no_mangle]
pub extern "C" fn golubegram_top_up_credits(amount: u64) {
    crate::billing::GLOBAL_BILLING.top_up(amount);
}
