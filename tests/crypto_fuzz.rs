use golubegram_core::crypto::RatchetSession;
use golubegram_core::storage::erasure::ErasureBroker;

#[test]
fn fuzz_double_ratchet_stub() {
    // In a real #![no_std] fuzzer, this would take arbitrary bytes from libfuzzer-sys
    let mut session = RatchetSession::new();
    let garbage_payload = vec![0xDE, 0xAD, 0xBE, 0xEF];
    
    // Attempting to encrypt garbage payload
    let encrypted = session.encrypt(&garbage_payload);
    assert!(!encrypted.is_empty(), "Fuzz: ratchet should encrypt payload without panic");
}

#[test]
fn fuzz_reed_solomon_stub() {
    let broker = ErasureBroker::new(10, 4);
    
    let simulated_file = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let fragments = broker.split_file("fuzz_target", simulated_file);
    assert_eq!(fragments.len(), 2, "Fuzz: RS broker should produce fragments");
}
