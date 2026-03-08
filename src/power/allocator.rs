use std::sync::Arc;
use std::collections::HashMap;
use crate::network::burn_protocol::BurnNotice;

/// Strict RAM Allocator for Hardware Offloading
/// Protects the user's NAND/SSD from thousands of random I/O writes during Sneakernet transit.

pub struct RamAllocator {
    max_capacity_bytes: usize,
    current_usage: usize,
    /// Chunks are pinned in RAM using Arc pointer. Stored with (Data, QoS Token Priority)
    buffers: HashMap<String, (Arc<[u8]>, u64)>,
}

impl RamAllocator {
    pub fn new(max_mb: usize) -> Self {
        Self {
            max_capacity_bytes: max_mb * 1024 * 1024,
            current_usage: 0,
            buffers: HashMap::new(),
        }
    }

    /// Allocate a chunk exclusively in RAM with QoS Priority. Drops traffic from quarantined nodes or lower priority transit.
    pub fn allocate(&mut self, id: &str, data: &[u8], sender_pubkey: Option<&[u8; 32]>, priority_credits: u64) -> Result<(), &'static str> {
        // SCORCHED EARTH PROTOCOL: Check Excommunication List
        if let Some(pubkey) = sender_pubkey {
            if BurnNotice::is_quarantined(pubkey) {
                return Err("SCORCHED EARTH ACTION: Dropping packet from quarantined node.");
            }
        }

        let size = data.len();
        if self.current_usage + size > self.max_capacity_bytes {
            // QoS Eviction logic
            // Find the buffer with the LOWEST priority credits
            let lowest_priority_item = self.buffers.iter()
                .min_by_key(|(_, (_, prio))| *prio)
                .map(|(k, (_, prio))| (k.clone(), *prio));
                
            if let Some((lowest_id, lowest_prio)) = lowest_priority_item {
                if priority_credits > lowest_prio {
                    // Evict lower priority transit to make room for paid tier
                    self.free(&lowest_id);
                } else {
                    return Err("RAM Allocator limit reached - Dropping packet (Insufficient QoS Tokens).");
                }
            } else {
                return Err("RAM Allocator limit reached.");
            }
        }

        let arc_data: Arc<[u8]> = Arc::from(data);
        self.buffers.insert(id.to_string(), (arc_data, priority_credits));
        self.current_usage += size;
        Ok(())
    }

    /// Retrieve a pinned RAM chunk
    pub fn get(&self, id: &str) -> Option<Arc<[u8]>> {
        self.buffers.get(id).map(|(arc, _)| arc.clone())
    }

    /// Free RAM manually
    pub fn free(&mut self, id: &str) {
        if let Some((arc, _)) = self.buffers.remove(id) {
            self.current_usage -= arc.len();
        }
    }
}
