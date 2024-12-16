use std::time::{Duration, Instant};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use local_ip_address::local_ip;
use serde::Serialize;

#[derive(Debug, Serialize)]  // Ensure this is serializable for Tauri
pub struct ServiceInfo {
    pub name: String,
    pub websocket_url: String,
}



#[tauri::command]
pub async fn discover_websocket() -> Result<Vec<ServiceInfo>, String> {
    let mdns = ServiceDaemon::new().map_err(|e| format!("Failed to create daemon: {}", e))?;
    let service_type = "_vortideck._tcp.local.";
    let receiver = mdns.browse(service_type).map_err(|e| format!("Failed to browse: {}", e))?;

    let timeout = Duration::from_secs(30);
    let start_time = Instant::now();
    let mut discovered_services = Vec::new(); // Collect all discovered services here

    while let Ok(event) = receiver.recv() {
        if start_time.elapsed() > timeout {
            println!("Timeout reached, stopping mDNS discovery...");
            break;
        }

        match event {
            ServiceEvent::ServiceResolved(info) => {
                // Stop browsing using the service type string
                mdns.stop_browse(service_type).map_err(|e| format!("Failed to stop browsing: {}", e))?;

                let ip = local_ip().map_err(|e| e.to_string())?;  // Get local IP address
                let port = info.get_port();
                let websocket_url = format!("ws://{}:{}/", ip, port);  // Construct WebSocket URL
                let service_name = info.get_fullname().to_string();  // Convert &str to String

                println!("WebSocket service found: {}", websocket_url);

                // Return the resolved service info

                 // Add the service info to the list
                discovered_services.push(ServiceInfo {
                    name: service_name.clone(),
                    websocket_url: websocket_url.clone(),
                });
                return Ok(discovered_services);
            }
            _ => {
                println!("Received mDNS event: {:?}", event); // Log other events
            }
        }
    }

    // If no service is found within the timeout, return an error
    Err("No WebSocket service found.".to_string())
}
