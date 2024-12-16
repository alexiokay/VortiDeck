use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;
use mdns::{self, RecordKind};
use serde::Serialize;
use std::net::IpAddr;
use std::pin::Pin;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize)] // Ensure it's serializable for Tauri
pub struct ServiceInfo {
    pub name: String,
    pub websocket_url: String,
}

#[tauri::command]
pub async fn discover_websocket() -> Result<Vec<ServiceInfo>, String> {
    // The service type we're searching for
    const SERVICE_NAME: &'static str = "_vortideck._tcp.local";
    const TIMEOUT_SECONDS: u64 = 1; // Timeout after 1 second

    let mut found_services = Vec::new();
    let mut seen_services = std::collections::HashSet::new(); // To track unique services

    // Provide a short query interval duration for mDNS discovery
    let stream = match mdns::discover::all(SERVICE_NAME, Duration::from_millis(100)) {
        // Short interval (100ms)
        Ok(stream) => Box::pin(stream.listen()), // Pin the stream
        Err(e) => return Err(format!("Failed to discover services: {}", e)),
    };

    let mut stream = stream.fuse(); // Fuse the stream to handle completion

    // Wait for and process responses from devices advertising the service
    let timeout = tokio::time::sleep(Duration::from_secs(TIMEOUT_SECONDS)); // Timeout Future
    tokio::select! {
        _ = timeout => {} // Wait for the timeout to expire
        _ = async {
            while let Some(Ok(response)) = stream.next().await {
                let hostname = response.hostname().unwrap_or("Unknown hostname");

                // Extract the IP address from the response records (A or AAAA records)
                if let Some(ip) = response.records().filter_map(to_ip_addr).next() {
                    // Handle Option<u16> for the port
                    let port = response.port().unwrap_or(0);
                    let websocket_url = format!("ws://{}:{}/", ip, port);

                    // Avoid adding duplicate services
                    if !seen_services.contains(&websocket_url) {
                        seen_services.insert(websocket_url.clone());
                        found_services.push(ServiceInfo {
                            name: hostname.to_string(),
                            websocket_url,
                        });
                    }
                } else {
                    println!("Service '{}' does not advertise an address", hostname);
                }
            }
        } => {}
    }

    // Return all found services, or an error if none were found
    if !found_services.is_empty() {
        Ok(found_services)
    } else {
        Err("No WebSocket services found.".to_string())
    }
}

// Convert mDNS record to an IP address (either IPv4 or IPv6)
fn to_ip_addr(record: &mdns::Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),    // IPv4
        RecordKind::AAAA(addr) => Some(addr.into()), // IPv6
        _ => None,                                   // Ignore other record types
    }
}
