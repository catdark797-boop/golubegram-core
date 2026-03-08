/// Web of Trust (WoT) implicit sybil mitigation
use std::collections::HashMap;
use std::sync::RwLock;

pub struct SybilShield;

lazy_static::lazy_static! {
    /// Tracks the 'Trust Score' of neighbor Anchor MACs.
    /// Newly seen MACs start with 0 trust. Valid routing interactions build trust over time.
    pub static ref WOT_REGISTRY: RwLock<HashMap<String, u32>> = RwLock::new(HashMap::new());
}

impl SybilShield {
    /// Calculate standard routing priority penalty based on Web Of Trust score.
    /// Mitigates Sybil / Flooding attacks from freshly spoofed MAC addresses.
    pub fn calculate_routing_penalty(mac_address: &str) -> u32 {
        let registry = WOT_REGISTRY.read().unwrap();
        
        match registry.get(mac_address) {
            Some(trust_score) if *trust_score > 100 => {
                // High trust node. No penalty.
                0
            },
            Some(trust_score) => {
                // Known but untrusted. Moderate penalty.
                (100 - trust_score) * 2 
            },
            None => {
                // Completely unknown MAC (Potential Sybil Node). MAXIMUM Routing penalty.
                println!("[SYBIL SHIELD] Unknown MAC {} detected. Applying maximum routing priority penalty.", mac_address);
                1000 
            }
        }
    }

    /// Increment trust upon successful cryptographic validation of a transit vector.
    pub fn reinforce_trust(mac_address: &str) {
        let mut registry = WOT_REGISTRY.write().unwrap();
        let counter = registry.entry(mac_address.to_string()).or_insert(0);
        if *counter < 100 {
            *counter += 1;
        }
    }
}
