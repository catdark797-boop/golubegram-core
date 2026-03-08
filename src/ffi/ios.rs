use crate::ffi::golubegram_init;

/// iOS specific C-FFI exports using standard C types
/// Handled by Swift using bridging headers.

#[no_mangle]
pub extern "C" fn ios_start_mesh_scan() {
    // Calls into mesh orchestrator
}
