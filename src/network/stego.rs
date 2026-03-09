/// LSB AUDIO STEGANOGRAPHY (DPI EVASION)
/// Embeds Cat_Dark AES-GCM encrypted payloads into the Least Significant Bits of standard VoIP / Voice Messages (`.ogg`, `.opus`).
/// Renders Mesh network traffic invisible to Deep Packet Inspection hardware.

pub struct LsbSteganography;

impl LsbSteganography {
    /// Encodes a raw encrypted payload into an audio buffer using 1-bit LSB steganography.
    /// Capacity: 1 bit per byte (e.g., 44.1kHz audio = ~43KB/s payload throughput).
    pub fn encode_payload_to_audio(mut audio_buffer: Vec<u8>, payload: &[u8]) -> Result<Vec<u8>, &'static str> {
        let capacity_bytes = audio_buffer.len() / 8;
        if payload.len() > capacity_bytes {
            return Err("Payload too large for this audio clip LSB capacity.");
        }

        let mut audio_idx = 0;
        
        for payload_byte in payload {
            for bit_idx in (0..8).rev() { // MSB to LSB iteration
                let bit = (payload_byte >> bit_idx) & 1;
                
                // Clear the least significant bit of the audio byte, then apply the payload bit
                audio_buffer[audio_idx] &= 0xFE; 
                audio_buffer[audio_idx] |= bit;
                
                audio_idx += 1;
            }
        }
        
        println!("[STEGO] Inserted {} bytes of Mesh Payload into {} bytes of VoIP Audio via LSB.", payload.len(), audio_buffer.len());
        Ok(audio_buffer)
    }

    /// Extracts the hidden payload from the LSBs of a received audio buffer.
    pub fn extract_payload_from_audio(audio_buffer: &[u8], payload_len: usize) -> Vec<u8> {
        let mut payload = Vec::with_capacity(payload_len);
        let mut audio_idx = 0;

        for _ in 0..payload_len {
            let mut extracted_byte = 0u8;
            for bit_idx in (0..8).rev() {
                let lsb = audio_buffer[audio_idx] & 1;
                extracted_byte |= lsb << bit_idx;
                audio_idx += 1;
            }
            payload.push(extracted_byte);
        }

        println!("[STEGO] Extracted {} bytes of hidden Mesh Payload from VoIP audio.", payload.len());
        payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsb_stego_roundtrip() {
        let fake_audio = vec![0x88; 1024]; // 1KB of "silence/noise"
        let secret_payload = b"INIT_CAT_DARK_HANDSHAKE:25519";
        
        let infected_audio = LsbSteganography::encode_payload_to_audio(fake_audio, secret_payload).unwrap();
        let extracted = LsbSteganography::extract_payload_from_audio(&infected_audio, secret_payload.len());
        
        assert_eq!(secret_payload.to_vec(), extracted, "LSB Steganography corruption occurred during extraction.");
    }
}
