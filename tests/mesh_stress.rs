use golubegram_core::network::mesh::{MeshOrchestrator, PeerState};

#[test]
fn test_mesh_90_percent_loss() {
    let mut orchestrator = MeshOrchestrator::new();
    
    // Simulate finding 100 peers
    for i in 0..100 {
        orchestrator.on_ble_discovered(format!("peer_{}", i), i % 2 == 0);
    }
    
    assert_eq!(orchestrator.peers.len(), 100);
    
    // Simulate 90% node loss (death, disconnect, out of range)
    for i in 0..90 {
        if let Some(mut peer) = orchestrator.peers.get_mut(&format!("peer_{}", i)) {
            peer.state = PeerState::Disconnected;
        }
    }
    
    let active_peers = orchestrator.peers.values().filter(|p| p.state != PeerState::Disconnected).count();
    assert_eq!(active_peers, 10);
    
    // Verify DTN is still functional via Hotspot
    assert!(orchestrator.is_hotspot_active, "Hotspot should remain active with remaining android donors");
}
