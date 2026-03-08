pub mod erasure;
pub mod evacuation;
pub mod stream;

// Main orchestrator for Level 2 (Память Роя)
pub struct StorageBroker {
    pub erasure: erasure::ErasureBroker,
    pub stream: stream::StreamBroker,
}

impl StorageBroker {
    pub fn new() -> Self {
        Self {
            erasure: erasure::ErasureBroker::new(10, 4),
            stream: stream::StreamBroker::new(),
        }
    }
}
