#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod create_websocket_mdns;
use create_websocket_mdns::create_websocket_mdns;
mod commands;
use tokio::time::{Duration, sleep};
use tauri::Manager;
use std::sync::Arc;
use std::collections::HashMap;
use futures_util::{StreamExt, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tokio::sync::broadcast;
use serde_json::Value;
use local_ip_address::local_ip;
use tokio::sync::Mutex as TokioMutex;
use tauri::State;
use std::sync::Mutex;
mod shared_state;
use crate::shared_state::AppState;

type PeerMap = Arc<TokioMutex<HashMap<String, PeerInfo>>>;

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
    let state: PeerMap = Arc::new(TokioMutex::new(HashMap::new()));
    let app_secrets = AppState::default();
    let system_state = Arc::new(Mutex::new(AppState::default()));

    let ws_secret_key = system_state.lock().unwrap().get_secret().unwrap_or_default(); // Fetch the secret

    let ip = local_ip().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

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

    let listener_clone = listener.clone();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    
    runtime.spawn(start_websocket(listener_clone.clone(), state.clone(), ws_secret_key.clone(), ip));

    let port = listener.local_addr().unwrap().port();
    println!("WebSocket server started on ws://{}:{}", ip, port);

    runtime.spawn(async move {
        if let Err(err) = create_websocket_mdns(ip.to_string(), port).await {
            eprintln!("Error in mDNS advertisement: {}", err);
        }
    });
    sleep(Duration::from_secs(10)).await;

    tauri::Builder::default()
        .manage(app_secrets)
        .manage(state)
        .manage(listener.clone())
        .invoke_handler(tauri::generate_handler![
            commands::generate_qr_code::generate_qr_code,
            commands::discover_websocket::discover_websocket
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}

async fn find_available_port() -> Result<Arc<TcpListener>, std::io::Error> {
    let listener = Arc::new(TcpListener::bind("0.0.0.0:0").await?);
    let port = listener.local_addr()?.port();
    println!("Listening on port {}", port);
    Ok(listener)
}

async fn start_websocket(listener: Arc<TcpListener>, state: PeerMap, ws_secret_key: String, ip: std::net::IpAddr) {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let state = state.clone();
                let ws_secret_key = ws_secret_key.clone();
                tokio::spawn(handle_connection(stream, state, ws_secret_key, ip));
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
    ws_secret_key: String,
    server_ip: std::net::IpAddr,
) {
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(_) => {
            eprintln!("Failed to get peer address");
            return;
        }
    };

    let peer_ip = peer_addr.ip().to_string();
    let mut client_id = "unknown".to_string();
    let mut device_type = "unknown".to_string();
    let mut provided_secret = None;

    let ws_stream = match accept_async(stream).await {
        Ok(ws_stream) => ws_stream,
        Err(e) => {
            eprintln!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = broadcast::channel::<Message>(100);

    if let Some(Ok(msg)) = read.next().await {
        if let Ok(text) = msg.to_text() {
            if let Ok(json) = serde_json::from_str::<Value>(text) {
                eprintln!("received: {}", text.secret);
                client_id = json["clientId"].as_str().unwrap_or("unknown").to_string();
                device_type = detect_device_type(json["device"].as_str().unwrap_or(""));
                provided_secret = json["secret"].as_str().map(|s| s.to_string());
            }
        }
    }

    // Validate secret for non-server IPs
    if peer_ip != server_ip.to_string() {
        let secrets_guard = secrets.lock().await;
        if let Some(secret) = provided_secret {
            if secrets_guard.get(&client_id) != Some(&secret) {
                eprintln!("Invalid or missing secret for client: {}", client_id);
                let _ = write.send(Message::Close(None)).await;
                return;
            }
        } else {
            eprintln!("No secret provided for client: {}", client_id);
            let _ = write.send(Message::Close(None)).await;
            return;
        }
    }

    println!("Client connected: {} ({})", client_id, device_type);
    // Add to peers map
    {
        let mut peers = state.lock().await;
        peers.insert(
            client_id.clone(),
            PeerInfo {
                sender: tx.clone(),
                device_type: device_type.clone(),
            },
        );
    }

    let client_id_clone = client_id.clone();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Ok(text) = msg.to_text() {
                println!("Received from {}: {}", client_id_clone, text);
            }
        }
    });

    while let Ok(msg) = rx.recv().await {
        if write.send(msg).await.is_err() {
            break;
        }
    }

    println!("Client disconnected: {}", client_id);
    let mut peers = state.lock().await;
    peers.remove(&client_id);
}

fn detect_device_type(user_agent: &str) -> String {
    if user_agent.contains("Mobile") {
        "mobile".to_string()
    } else {
        "desktop".to_string()
    }
}


async fn forward_to_mobile(state: PeerMap, payload: String) {
    let peers = state.lock().await;
    for (client_id, peer) in peers.iter() {
        if peer.device_type == "mobile" {
            let _ = peer.sender.send(Message::Text(payload.clone()));
        }
    }
}