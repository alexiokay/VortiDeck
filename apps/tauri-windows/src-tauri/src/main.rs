// src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands; 
mod create_websocket_mdns;
use create_websocket_mdns::create_websocket_mdns; // Correct path to the function

use tauri::Manager;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use futures_util::{StreamExt, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tokio::sync::broadcast;
use tauri::State;
use serde_json::Value;
use local_ip_address::local_ip;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use mdns_sd::{ServiceDaemon, ServiceInfo};

use std::any::Any;
use zeroconf::prelude::*;
use zeroconf::{MdnsService, ServiceRegistration, ServiceType, TxtRecord};
use tokio::time::{Duration, sleep};

use std::task::Context as StdContext;
use futures_util::task::Context as FutContext;
use tauri::Context as TauriContext;
// use tokio::signal;  // For graceful shutdown on SIGINT and SIGTERM

use tokio::runtime;

use tokio::runtime::Builder; // Ensure you're importing the correct `Builder`

use std::process::Command;

type PeerMap = Arc<Mutex<HashMap<String, PeerInfo>>>; // Using clientId as String for keys

#[derive(Debug)]
struct MyContext {
    service_name: String,
}
#[derive(Debug, Clone)]
struct PeerInfo {
    sender: broadcast::Sender<Message>,
    device_type: String, // "mobile" or "desktop"
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[tokio::main]
async fn main() {
  
    

    // The main runtime to handle your logic
    let state: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    // Get the local IP address
    let ip = local_ip().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

    // Try to bind to port 9001, if it fails, find an available port
    let listener = match TcpListener::bind(format!("{}:9001", ip)).await {
        Ok(listener) => Arc::new(listener),
        Err(_) => {
            println!("Port 9001 is unavailable, finding an available port...");
            match find_available_port().await {
                Ok(listener) => listener,
                Err(e) => {
                    eprintln!("Error finding an available port: {}", e);
                    return;
                }
            }
        }
    };

    // Clone the listener before passing it into the spawned task
    let listener_clone = listener.clone();

    // Create a custom runtime using Builder
    let runtime = Builder::new_multi_thread()
        .worker_threads(2) // Adjust the number of threads as needed
        .enable_all() // Enable all Tokio features (like signal handling)
        .build()
        .unwrap();


    
    // Spawn the WebSocket server in the custom runtime
    runtime.spawn(start_websocket(listener_clone.clone(), state.clone()));

    // Get the actual port the server is listening on
    let port = listener.local_addr().unwrap().port();
    println!("WebSocket server started on ws://{}:{}", ip, port);

    // Start the mDNS advertising task in the custom runtime
    let ip = "192.168.178.129".to_string();
    let port = 9001;

    // Run mDNS advertisement in the background
    runtime.spawn(async move {
        if let Err(err) = create_websocket_mdns(ip, port).await {
            eprintln!("Error in mDNS advertisement: {}", err);
        }
    });
    sleep(Duration::from_secs(10)).await;

  

    // Clone the listener before using it with Tauri
    tauri::Builder::default()
        .manage(state)
        .manage(listener.clone()) // Use the cloned listener here as well
        .invoke_handler(tauri::generate_handler![commands::discover_websocket::discover_websocket])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}

async fn find_available_port() -> Result<Arc<TcpListener>, std::io::Error> {
    // Create a TcpListener bound to a random available port
    let listener = Arc::new(TcpListener::bind("0.0.0.0:0").await?);
    let port = listener.local_addr()?.port();
    println!("Listening on port {}", port);
    Ok(listener)
}

async fn start_websocket(listener: Arc<TcpListener>, state: PeerMap) {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream, state.clone())); // Remove id handling
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                break;
            }
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    state: PeerMap,
) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = broadcast::channel::<Message>(100);

    // Read the first message to extract clientId and device type
    let mut client_id = "unknown".to_string();
    let mut device_type = "unknown".to_string();

    if let Some(Ok(msg)) = read.next().await {
        if let Ok(text) = msg.to_text() {
            if let Ok(json) = serde_json::from_str::<Value>(text) {
                if let Some(id) = json["clientId"].as_str() {
                    client_id = id.to_string(); // Get clientId from incoming message
                }
                if let Some(device) = json["device"].as_str() {
                    device_type = detect_device_type(device);
                }
            }
        }
    }

    println!("New connection: clientId={}, device={}", client_id, device_type);

    // Insert or replace existing connection if the clientId already exists
    {
        let mut peers = state.lock().unwrap();
        if peers.contains_key(&client_id) {
            println!("Replacing existing connection for clientId: {}", client_id);
        }
        peers.insert(client_id.clone(), PeerInfo { sender: tx.clone(), device_type: device_type.clone() });
    }

    // Spawn a task to handle incoming messages
    let state_clone = state.clone();
    let client_id_clone = client_id.clone(); // Clone the client_id here
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Ok(text) = msg.to_text() {
                println!("Received from {}: {}", client_id_clone, text); // Use cloned client_id
                // Parse the message as JSON
                if let Ok(json) = serde_json::from_str::<Value>(text) {
                    if let Some(action) = json["action"].as_str() {
                        if action == "component" {
                            let payload = json.to_string();
                            // Forward the message to all mobile devices
                            forward_to_mobile(state_clone.clone(), payload);
                        }
                    }
                }
            }
        }
    });

    // Handle outgoing messages
    while let Ok(msg) = rx.recv().await {
        if write.send(msg).await.is_err() {
            break;
        }
    }

    // Send a close frame when we're done
    let _ = write.send(Message::Close(None)).await;
    // Clean up when the client disconnects
    {
        let mut peers = state.lock().unwrap();
        peers.remove(&client_id); // Correct use of client_id as the key (String)
    }
    println!("Connection with clientId {} closed", client_id);
}

fn detect_device_type(user_agent: &str) -> String {
    if user_agent.contains("Mobile") {
        "mobile".to_string()
    } else if user_agent.contains("Windows") || user_agent.contains("Macintosh") || user_agent.contains("Linux") {
        "desktop".to_string()
    } else {
        "unknown".to_string()
    }
}

fn forward_to_mobile(state: PeerMap, payload: String) {
    let peers = state.lock().unwrap();

    for (client_id, peer) in peers.iter() {
        if peer.device_type == "mobile" {
            println!("Forwarding to mobile device with clientId: {}", client_id);
            let _ = peer.sender.send(Message::text(payload.clone()));
            // let _ = peer.sender.send(Message::Close(None));

        }
    }
}

// Gracefully shutdown WebSocket connections
async fn shutdown_websocket_connections(state: PeerMap) {
    let peers = state.lock().unwrap();
    for (client_id, peer) in peers.iter() {
        // Send a close message to each connected client
        println!("Closing connection for clientId: {}", client_id);
        let _ = peer.sender.send(Message::Close(None));
    }

    // Clean up peers after sending close messages
    println!("WebSocket server shutting down...");
}





