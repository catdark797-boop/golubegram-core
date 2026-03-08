use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Daemon] Golubegram Privileged Mesh Daemon starting...");
    
    // In production, this would be /var/run/golubegram-mesh.sock
    // We use /tmp for development to avoid requiring root for the socket creation itself,
    // though the daemon holds CAP_NET_RAW for BlueZ.
    let socket_path = "/tmp/golubegram-mesh.sock";
    
    // Cleanup old socket if it exists
    let _ = fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path)?;
    println!("[Daemon] Listening on IPC socket: {}", socket_path);
    
    // Ensure the socket is accessible to unprivileged users
    // In a real scenario, we'd set ACLs restrictively via chown
    let _ = fs::set_permissions(socket_path, std::os::unix::fs::PermissionsExt::from_mode(0o666));

    loop {
        let (mut stream, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match stream.read(&mut buf).await {
                    Ok(0) => break, // Connection closed
                    Ok(n) => {
                        println!("[Daemon] Received {} bytes from unprivileged app. Routing to raw BLE socket...", n);
                        // SIMULATED: Here the daemon interacts with raw BlueZ sockets 
                        // as it holds `CAP_NET_RAW`
                        
                        // Echo success back
                        if let Err(e) = stream.write_all(b"BLE_TX_OK").await {
                            eprintln!("[Daemon] Failed to answer IPC: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("[Daemon] IPC Read error: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
