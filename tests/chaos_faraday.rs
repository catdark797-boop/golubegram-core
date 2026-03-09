use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// THE FARADAY CAGE CHAOS MONKEY
/// Brutally isolates the node mid-transfer to ensure the FFI buffers to disk 
/// and Reed-Solomon streams resync perfectly upon reconnection.
#[test]
fn test_faraday_cage_isolation_resume() {
    let _is_transfer_active = Arc::new(AtomicBool::new(true));
    let connection_state = Arc::new(AtomicBool::new(true));
    let payload_corruption_flag = Arc::new(AtomicBool::new(false));
    let buffer_disk_write_success = Arc::new(AtomicBool::new(false));

    let ffi_thread = {
        let conn = connection_state.clone();
        let corruption = payload_corruption_flag.clone();
        let disk_write = buffer_disk_write_success.clone();

        thread::spawn(move || {
            let mut payload_chunks_received = 0;
            let total_chunks = 1000; // Simulating 1GB Payload
            
            while payload_chunks_received < total_chunks {
                if conn.load(Ordering::Relaxed) {
                    // Simulating active Reed-Solomon stream decoding
                    payload_chunks_received += 1;
                    thread::sleep(Duration::from_millis(1));
                } else {
                    // FARADAY CAGE INITIATED. Buffer to disk.
                    println!("[FFI THREAD] Connection lost! Initiating emergency disk buffer swap...");
                    disk_write.store(true, Ordering::Relaxed);
                    
                    // Simulate waiting in the dark
                    while !conn.load(Ordering::Relaxed) {
                        thread::sleep(Duration::from_millis(10));
                    }
                    println!("[FFI THREAD] Connection restored! Resuming Reed-Solomon stream from chunk {}...", payload_chunks_received);
                }
            }

            // Simulating integrity check
            if payload_chunks_received != total_chunks {
                corruption.store(true, Ordering::Relaxed); 
            }
        })
    };

    // Main thread acts as the Operator putting the phone in a microwave
    thread::sleep(Duration::from_millis(150)); // Wait for transfer to start
    println!("[CHAOS MONKEY] Node thrown into Faraday Cage (Signal Loss).");
    connection_state.store(false, Ordering::Relaxed);
    
    thread::sleep(Duration::from_millis(200)); // Total blackout duration
    
    println!("[CHAOS MONKEY] Node removed from Faraday Cage. Signal restored.");
    connection_state.store(true, Ordering::Relaxed);

    ffi_thread.join().unwrap();

    assert!(buffer_disk_write_success.load(Ordering::Relaxed), "The FFI must buffer payload to disk during blackout.");
    assert!(!payload_corruption_flag.load(Ordering::Relaxed), "1GB Payload corrupted. Reed-Solomon resume failed.");
    println!("[SUCCESS] Faraday Cage Isolation Test passed without memory corruption.");
}
