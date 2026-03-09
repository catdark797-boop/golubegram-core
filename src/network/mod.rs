pub mod mesh;
pub mod stealth;
pub mod awdl;
pub mod ota;
pub mod ntp;
pub mod sneakernet;
pub mod ipc;
pub mod router;
pub mod burst;
pub mod burn_protocol;
pub mod anti_trace;
pub mod battery;
pub mod anchor;
pub mod infection;
pub mod tor_bridge;
pub mod stego;

// Main network state machine and orchestration
pub struct NetBroker {
    pub mesh: mesh::MeshOrchestrator,
    pub ntp: ntp::NtpState,
}

impl NetBroker {
    pub fn new() -> Self {
        Self {
            mesh: mesh::MeshOrchestrator::new(),
            ntp: ntp::NtpState::new(),
        }
    }
}
