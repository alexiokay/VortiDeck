use std::collections::HashSet;
use std::time::{Duration, Instant};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use local_ip_address::local_ip;
use serde::Serialize;
use tauri::command;
use tokio::sync::mpsc;

/// Represents a discovered WebSocket service.
#[derive(Debug, Serialize, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub websocket_url: String,
}

/// Discovers WebSocket services using mDNS in the background.

#[tauri::command]
pub async fn discover_websocket() -> Result<Vec<ServiceInfo>, String> {
    // Create a channel to receive discovered services
    let (tx, mut rx) = mpsc::channel(10);

    // Spawn the discovery process in the background
    tokio::spawn(async move {
        let result = discover_services(tx).await;
        if let Err(e) = result {
            println!("Error during mDNS discovery: {}", e);
        }
    });

    // Collect services received from the channel
    let mut discovered_services = Vec::new();
    while let Some(service) = rx.recv().await {
        discovered_services.push(service);
    }

    if discovered_services.is_empty() {
        Err("No WebSocket services found.".to_string())
    } else {
        Ok(discovered_services)
    }
}

/// Handles the mDNS discovery process.
async fn discover_services(tx: mpsc::Sender<ServiceInfo>) -> Result<(), String> {
    let mdns = ServiceDaemon::new().map_err(|e| format!("Failed to create mDNS daemon: {}", e))?;
    let service_type = "_vortideck._tcp.local.";
    let receiver = mdns.browse(service_type).map_err(|e| format!("Failed to browse services: {}", e))?;

    let timeout = Duration::from_secs(30);
    let start_time = Instant::now();
    let mut discovered_set = HashSet::new(); // Deduplication using a HashSet

    println!("Starting mDNS discovery for type: {}", service_type);

    while let Ok(event) = receiver.recv() {
        if start_time.elapsed() > timeout {
            println!("Timeout reached, stopping mDNS discovery...");
            break;
        }

        match event {
            ServiceEvent::ServiceResolved(info) => {
                // println!("Service resolved: {:?}", info);

                // Get local IP address
                let ip = match local_ip() {
                    Ok(ip) => ip.to_string(),
                    Err(e) => {
                        println!("Failed to get local IP address: {}", e);
                        continue;
                    }
                };

                // Build the WebSocket URL
                let port = info.get_port();
                let websocket_url = format!("ws://{}:{}/", ip, port);
                let service_name = info.get_fullname().to_string();

                // Deduplicate using the fullname
                if discovered_set.insert(service_name.clone()) {
                    println!("WebSocket service found: {}", websocket_url);

                    let service_info = ServiceInfo {
                        name: service_name,
                        websocket_url,
                    };

                    if tx.send(service_info).await.is_err() {
                        println!("Receiver dropped; stopping discovery...");
                        break;
                    }
                    break;
                }
            }
            ServiceEvent::ServiceRemoved(..) => {
                println!("Service removed: {:?}", event);
            }
            _ => {
                // println!("Received other mDNS event: {:?}", event);
            }
        }
    }

    if let Err(e) = mdns.stop_browse(service_type) {
        println!("Warning: Failed to stop browsing: {}", e);
    }

    println!("mDNS discovery finished.");

    Ok(())
}
