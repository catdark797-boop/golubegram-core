/// IDENTICONS: ED25519 Visual Representation
/// Deterministically maps a raw public key to a geometric colored avatar.

use sha2::{Sha256, Digest};

pub struct IdenticonGenerator;

impl IdenticonGenerator {
    /// Generates a visual identicon hash profile from a Public Key
    pub fn generate_from_pubkey(ed25519_pubkey: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(ed25519_pubkey);
        let result = hasher.finalize();
        
        // Extract 3 bytes for RGB background color
        let r = result[0];
        let g = result[1];
        let b = result[2];

        // Extract 1 byte for geometric shape index
        let shape_index = result[3] % 5;
        let shapes = ["Circle", "Square", "Hexagon", "Triangle", "Diamond"];

        let identicon_meta = format!("[Identicon: Color(#{:02x}{:02x}{:02x}), Shape({})]", r, g, b, shapes[shape_index as usize]);
        println!("[IDENTITY] Generated minimal avatar for {}: {}", ed25519_pubkey, identicon_meta);
        
        identicon_meta
    }
}
