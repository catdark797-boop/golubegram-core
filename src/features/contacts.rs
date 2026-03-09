/// PRIVACY-PRESERVING CONTACT SYNC
/// Cross-references OS phonebooks using SHA-256 hashes without exposing raw PII.

use sha2::{Sha256, Digest};
use std::collections::HashMap;

pub struct ContactManager;

impl ContactManager {
    /// Hashes the raw OS Address Book (never leaves the device)
    pub fn hash_local_address_book(raw_contacts: &HashMap<String, String>) -> HashMap<String, String> {
        let mut hashed_book = HashMap::new();
        
        // Key: Name, Value: Phone
        for (name, phone) in raw_contacts {
            let mut hasher = Sha256::new();
            // Normalize phone (strip + and spaces)
            let normalized = phone.replace("+", "").replace(" ", "");
            hasher.update(normalized.as_bytes());
            let hash_hex = format!("{:x}", hasher.finalize());
            
            hashed_book.insert(hash_hex, name.clone());
        }
        
        hashed_book
    }

    /// Intersects received Mesh node phone hashes against the local hashed phonebook
    pub fn intersect_and_resolve(incoming_hash: &str, local_hashed_book: &HashMap<String, String>) -> Option<String> {
        if let Some(real_name) = local_hashed_book.get(incoming_hash) {
            println!("[CONTACT SYNC] Match found! Overriding Mesh Alias with OS Contact Name: '{}'", real_name);
            Some(real_name.clone())
        } else {
            None
        }
    }
}
