/// Apple Wireless Direct Link (AWDL) Bridge
/// Used for pure iOS-to-iOS MultipeerConnectivity fallback.

pub trait AwdlDelegate {
    fn start_session(&self, session_id: &str);
    fn send_data(&self, peer_id: &str, data: &[u8]);
}
