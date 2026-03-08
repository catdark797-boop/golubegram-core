/// P2P Smart-Streaming for Heavy Media
use std::collections::HashMap;

pub struct StreamBroker {
    /// Active lightweight copies being assembled in RAM
    active_streams: HashMap<String, Vec<u8>>,
}

impl StreamBroker {
    pub fn new() -> Self {
        Self {
            active_streams: HashMap::new(),
        }
    }

    /// Feeds incoming fragments into the video stream buffer while original downloads
    pub fn push_stream_fragment(&mut self, file_id: &str, data: &[u8]) {
        let stream = self.active_streams.entry(file_id.to_string()).or_insert(Vec::new());
        stream.extend_from_slice(data);
    }

    /// Read available stream bytes for native player
    pub fn read_stream(&self, file_id: &str) -> Option<&[u8]> {
        self.active_streams.get(file_id).map(|v| v.as_slice())
    }
}
