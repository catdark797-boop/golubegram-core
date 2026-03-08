use super::CryptoError;

/// Trait defining the FFI boundary for Hardware-Backed Keystore
/// (Apple Secure Enclave / Android Keystore)
pub trait HardwareKeystore {
    fn generate_keypair(&self) -> Result<Vec<u8>, CryptoError>;
    fn sign_payload(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn decrypt_payload(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError>;
}
