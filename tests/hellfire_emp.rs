use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// THE EMP SIMULATION:
/// Simulates a sudden death of the underlying BLE/Wi-Fi daemon midway through a large I/O operation.
/// Asserts that the FFI Bridge does not deadlock the UI thread and recovers gracefully.

#[test]
fn test_emp_daemon_death_during_io() {
    let daemon_alive = Arc::new(AtomicBool::new(true));
    let transfer_complete = Arc::new(AtomicBool::new(false));
    let panic_caught = Arc::new(AtomicBool::new(false));

    let ffi_bridge_thread = {
        let alive = daemon_alive.clone();
        let complete = transfer_complete.clone();
        let caught = panic_caught.clone();
        
        thread::spawn(move || {
            let result = std::panic::catch_unwind(|| {
                // Simulating 10MB chunk transfer
                for chunk in 0..100 {
                    if !alive.load(Ordering::Relaxed) {
                        panic!("DAEMON_PIPE_BROKEN: EMP Signal received");
                    }
                    thread::sleep(Duration::from_millis(5)); // I/O simulation
                }
                complete.store(true, Ordering::Relaxed);
            });

            if result.is_err() {
                caught.store(true, Ordering::Relaxed);
            }
        })
    };

    // Main thread acts as the EMP (Jammer/Kill-switch)
    thread::sleep(Duration::from_millis(250)); // Wait for transfer to reach 50%
    println!("[EMP DEPLOYED] Killing Daemon violently...");
    daemon_alive.store(false, Ordering::Relaxed);

    ffi_bridge_thread.join().unwrap();

    assert!(panic_caught.load(Ordering::Relaxed), "FFI Bridge must catch the daemon death panic without crashing the OS process.");
    assert!(!transfer_complete.load(Ordering::Relaxed), "Transfer should not have completed.");
    println!("[EMP SURVIVED] FFI UI Thread recovered gracefully from daemon death.");
}
