use std::collections::VecDeque;

/// A payload stored temporarily on the device for Delay-Tolerant Networking
#[derive(Clone)]
pub struct SneakernetPayload {
    pub destination_pubkey_hash: String,
    pub encrypted_blob: Vec<u8>,
    pub ttl_blocks: u32,
}

pub struct SneakernetQueue {
    queue: VecDeque<SneakernetPayload>,
    max_ram_bytes: usize,
    current_bytes: usize,
}

impl SneakernetQueue {
    pub fn new(max_ram_mb: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            max_ram_bytes: max_ram_mb * 1024 * 1024,
            current_bytes: 0,
        }
    }

    /// Stores a payload STRICTLY in RAM (Hardware Offloading to protect SSD)
    pub fn enqueue(&mut self, payload: SneakernetPayload) -> Result<(), &'static str> {
        let size = payload.encrypted_blob.len();
        if self.current_bytes + size > self.max_ram_bytes {
            return Err("RAM Queue Full - Cannot accept Sneakernet payload");
        }
        
        self.current_bytes += size;
        self.queue.push_back(payload);
        Ok(())
    }

    /// Extracts payloads intended for a specific peer
    pub fn extract_for_peer(&mut self, pubkey_hash: &str) -> Vec<SneakernetPayload> {
        // In reality, this would filter. For V1.0 structure:
        let mut extracted = Vec::new();
        let mut remaining = VecDeque::new();
        
        while let Some(msg) = self.queue.pop_front() {
            if msg.destination_pubkey_hash == pubkey_hash {
                self.current_bytes -= msg.encrypted_blob.len();
                extracted.push(msg);
            } else {
                remaining.push_back(msg);
            }
        }
        self.queue = remaining;
        extracted
    }
}
