use golubegram_core::network::burn_protocol::BurnNotice;
use golubegram_core::power::allocator::RamAllocator;
use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;

#[test]
fn test_insider_threat_burn_protocol() {
    let mut csprng = OsRng {};
    let admin_keypair = Keypair::generate(&mut csprng);
    
    // Compromised device
    let traitor_keypair = Keypair::generate(&mut csprng);
    let traitor_pubkey = traitor_keypair.public.to_bytes();
    
    // 1. Generate the Burn Notice payload
    let timestamp = 1710000000;
    let mut msg_bytes = Vec::new();
    msg_bytes.extend_from_slice(&traitor_pubkey);
    msg_bytes.extend_from_slice(&timestamp.to_be_bytes());
    
    // Signed by Admin Root CA
    let admin_signature = admin_keypair.sign(&msg_bytes);
    
    let notice = BurnNotice {
        targeted_pubkey: traitor_pubkey,
        timestamp,
        admin_signature,
    };
    
    // 2. Validate Burn Notice Processing
    let is_new_notice = BurnNotice::process_notice(&notice, &admin_keypair.public).unwrap();
    assert!(is_new_notice, "First notice should trigger broadcast");
    
    // Prevent infinite broadcast loops
    let is_duplicate = BurnNotice::process_notice(&notice, &admin_keypair.public).unwrap();
    assert!(!is_duplicate, "Cache should prevent duplicate broadcast triggered");
    
    // 3. Excommunication (Allocator Test)
    let mut allocator = RamAllocator::new(32);
    
    // Try to allocate some transit memory originating from the traitor
    let test_packet = b"Malicious payload trying to pivot";
    let allocation_result = allocator.allocate("transit_1", test_packet, Some(&traitor_pubkey), 10);
    
    assert!(
        allocation_result.is_err(), 
        "CRITICAL VULNERABILITY: Allocator accepted transit traffic from a burned node!"
    );
    
    let normal_pubkey = Keypair::generate(&mut csprng).public.to_bytes();
    let normal_result = allocator.allocate("transit_2", b"Good mesh traffic", Some(&normal_pubkey), 10);
    
    assert!(
        normal_result.is_ok(),
        "Allocator dropped legitimate traffic!"
    );
    
    println!("[SCORCHED EARTH] Insider Threat Defused. Quarantine Active. Traffic Dropped.");
}
