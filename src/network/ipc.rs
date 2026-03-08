#[cfg(target_family = "unix")]
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct MeshIpcClient {
    socket_path: String,
}

impl MeshIpcClient {
    pub fn new() -> Self {
        Self {
            socket_path: "/tmp/golubegram-mesh.sock".to_string(),
        }
    }

    #[cfg(target_family = "unix")]
    pub async fn transmit_raw(&self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Connect as unprivileged user to the root daemon
        let mut stream = UnixStream::connect(&self.socket_path).await?;
        stream.write_all(payload).await?;
        
        let mut buf = vec![0; 32];
        let n = stream.read(&mut buf).await?;
        
        if &buf[..n] == b"BLE_TX_OK" {
            Ok(())
        } else {
            Err("NACK from Privileged Daemon".into())
        }
    }

    #[cfg(not(target_family = "unix"))]
    pub async fn transmit_raw(&self, _payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Windows/Mac handle native APIs directly inside the main UI process context
        Ok(())
    }
}
