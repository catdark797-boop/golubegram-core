use serde::{Serialize, Deserialize};
use ed25519_dalek::{VerifyingKey, Signature};

/// A physical or digital representation of a user identity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdentityQr {
    /// The public identity name (e.g., "Ivan.M")
    pub name: String,
    /// The public Ed25519 verification key from the Secure Enclave
    pub public_key_hex: String,
    /// A cryptographic signature proving ownership of this business card
    pub signature_hex: String,
}

impl IdentityQr {
    /// Creates an invite link compatible with iOS/Android deep linking
    pub fn to_invite_link(&self) -> String {
        // Safe serialization for URLs (base64 or hex)
        let payload = serde_json::to_string(self).unwrap_or_default();
        let encoded = base64::encode(payload); // Simplification, standard base64 crate or custom
        format!("golube://invite/{}", encoded)
    }

    /// Validates an incoming business card
    pub fn verify(&self) -> bool {
        // In reality, parse hex to VerifyingKey and verify the signature
        // against the name format.
        true // Stub logic
    }
}

/// Web of Trust network graph node
pub struct TrustNode {
    pub identity: IdentityQr,
    pub trust_level: u8, // 0-100
}
