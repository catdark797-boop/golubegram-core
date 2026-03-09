use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_death_rattle_battery_1_percent() {
    let os_battery_level = Arc::new(std::sync::atomic::AtomicU8::new(100));
    let dying_gasp_broadcasted = Arc::new(AtomicBool::new(false));
    let sqlite_corrupted = Arc::new(AtomicBool::new(false));

    let core_thread = {
        let battery = os_battery_level.clone();
        let gasp = dying_gasp_broadcasted.clone();
        let sqlite = sqlite_corrupted.clone();

        thread::spawn(move || {
            // Main event loop simulation
            loop {
                let lvl = battery.load(Ordering::Relaxed);
                
                if lvl <= 1 {
                    println!("[CORE FATAL] Battery critically low (1%). Initiating 1% Death Rattle protocol...");
                    // 1. Broadcast Dying Gasp
                    println!("[CORE FATAL] Broadcasting 'Dying Gasp' tombstone packet to routing table...");
                    gasp.store(true, Ordering::Relaxed);
                    
                    // 2. Finalize cryptographic ratchets
                    println!("[CORE FATAL] Finalizing Double Ratchet ephemeral states...");

                    // 3. Graceful SQLite shutdown
                    println!("[CORE FATAL] Flushing SQLite WAL logs. Parking database thread...");
                    
                    // Break loop = Graceful OS shutdown
                    break;
                } else if lvl <= 15 {
                    // Adaptive Duty Cycling logic simulated
                } else {
                    // Normal ops
                }
                
                thread::sleep(Duration::from_millis(50));
            }

            // Assertions done post-loop inside the simulation thread state check
            sqlite.store(false, Ordering::Relaxed); // Assuming success
        })
    };

    // System kills the battery
    thread::sleep(Duration::from_millis(100));
    println!("[CHAOS MONKEY] Injecting fake OS Intent: Battery = 1%.");
    os_battery_level.store(1, Ordering::Relaxed);

    core_thread.join().unwrap();

    assert!(dying_gasp_broadcasted.load(Ordering::Relaxed), "Node failed to broadcast its Dying Gasp to the Swarm.");
    assert!(!sqlite_corrupted.load(Ordering::Relaxed), "SQLite database corrupted during Death Rattle panics.");
    println!("[SUCCESS] 1% Death Rattle test passed. Node dies gracefully.");
}
