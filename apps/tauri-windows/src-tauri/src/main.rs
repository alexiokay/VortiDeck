#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod create_websocket_mdns;
use create_websocket_mdns::create_websocket_mdns;
mod commands;
use tokio::time::{Duration, sleep, timeout};
use tauri::Manager;
use std::sync::Arc;
use std::collections::HashMap;
use futures_util::{StreamExt, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tokio::sync::broadcast;
use serde_json::Value;
use local_ip_address::local_ip;
use tauri::State;
use std::sync::Mutex;
mod shared_state;
use crate::shared_state::AppState;
use crate::shared_state::{PeerState, PeerInfo};

use tauri::App;
use tauri::{AppHandle, Emitter};
use std::net::IpAddr;
use serde::Serialize;
use platform_info::*;
use serde_json::json;

// type PeerState = Arc<TokioMutex<HashMap<String, PeerInfo>>>;

// #[derive(Debug, Clone)]
// struct PeerInfo {
//     client_id: String, // Add client_id here
//     sender: broadcast::Sender<Message>,
//     device_type: String, // "mobile" or "desktop"
//     device: String,
// }

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SerializablePeerInfo  {
    client_id: String,
    device_type: String, // Keep only serializable fields
    device: String,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use std::thread;

    // let test = Manager::state::<AppState>(app);
    let app_handle = app.handle().clone();

    // test.display_secret();

    // Start async tasks
    tauri::async_runtime::spawn(async move {
        my_app(app_handle).await
    });


    Ok(())
}



async fn my_app(app_handle: AppHandle) {

    

    // let state: PeerState = Arc::new(TokioMutex::new(HashMap::new()));

    // let test = app_handle.state::<AppState>();

    // let ws_secret_key = test.get_secret().unwrap_or_default(); // Fetch the secret

    let ip = local_ip().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

    let listener = match TcpListener::bind(format!("{}:9001", ip)).await {
        Ok(listener) => Arc::new(listener),
        Err(_) => {
            println!("Port 9001 is unavailable, finding an available port...");
            match find_available_port().await {
                Ok(listener) => listener,
                Err(e) => {
                    eprintln!("Error finding an available port: {}", e);
                    return; // Return the error here
                }
            }
        }
    };
    let port = listener.local_addr().unwrap().port();

    let app_state = app_handle.state::<AppState>();
    let info = PlatformInfo::new().expect("Unable to determine platform info");
    app_state.set_server_data(ip, port, info.sysname().to_string_lossy().to_string(), info.nodename().to_string_lossy().to_string());
    // let runtime = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(2)
    //     .enable_all()
    //     .build()
    //     .unwrap();



    let listener_clone = listener.clone();

    tauri::async_runtime::spawn(start_websocket(listener_clone, app_handle, ip));

    
    println!("WebSocket server started on ws://{}:{}", ip, port);

    tauri::async_runtime::spawn(async move {
        if let Err(err) = create_websocket_mdns(ip.to_string(), port).await {
            eprintln!("Error in mDNS advertisement: {}", err);
        }
    });
    
    // sleep(Duration::from_secs(10)).await;
 

}

#[tokio::main]
async fn main() {
   
    let app_state = AppState::default();
    let peer_state = PeerState::default();
    

   



   
    
    tauri::Builder::default()
        .setup(setup)
        .manage(app_state)
        .manage(peer_state)
        // .manage(state)
        // .manage(listener.clone())
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

async fn start_websocket(listener: Arc<TcpListener>, app_handle: AppHandle, ip: std::net::IpAddr) {
    loop {
        // println!("test {}", app_state.get_secret());
        match listener.accept().await {
            
            Ok((stream, _)) => {
            
              
                tokio::spawn(handle_connection(stream, app_handle.clone(), ip));
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
    app_handle: AppHandle,
    server_ip: IpAddr,
) {
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(_) => {
            eprintln!("Failed to get peer address");
            return;
        }
    };

    let app_state = app_handle.state::<AppState>();
    let state = app_handle.state::<PeerState>();

    let peer_ip = peer_addr.ip().to_string();
    let mut client_id = "unknown".to_string();
    let mut device_type = "unknown".to_string();
    let mut device_model = "unknown".to_string();
    let mut provided_secret = None;

   

    //This ensures that if the handshake takes longer than 5 seconds, the server will abort.
    let ws_stream = match timeout(Duration::from_secs(5), accept_async(stream)).await {
        Ok(Ok(ws_stream)) => ws_stream,
        Ok(Err(e)) => {
            eprintln!("WebSocket handshake failed: {}", e);
            return;
        }
        Err(_) => {
            eprintln!("WebSocket handshake timed out");
            return;
        }
    };

    // // old -> 
    // let ws_stream = match accept_async(stream).await {
    //     Ok(ws_stream) => ws_stream,
    //     Err(e) => {
    //         eprintln!("WebSocket handshake failed: {}", e);
    //         return;
    //     }
    // };

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = broadcast::channel::<Message>(2000); // Increased buffer size
   
    // Parse initial client message
    if let Some(Ok(msg)) = read.next().await {
        if let Ok(text) = msg.to_text() {
            if let Ok(json) = serde_json::from_str::<Value>(text) {
                eprintln!("Received initial message: {}", text);
                client_id = json["clientId"].as_str().unwrap_or("unknown").to_string();
                device_type = detect_device_type(json["device"]["device"]["type"].as_str().unwrap_or(""));
                device_model = format!(
                    "{}",json["device"]["device"]["model"].as_str().unwrap_or("")
                );
                
                provided_secret = json["secret"].as_str().map(|s| s.to_string());
            }
        }
    }

    // Authenticate client
    if peer_ip != server_ip.to_string() {
        match provided_secret {
            Some(secret) if Some(&secret) == app_state.get_secret().as_ref() => {
                eprintln!("Client authenticated: {}", client_id);
            }
            _ => {
                eprintln!("Authentication failed for client: {}", client_id);
                let _ = write.send(Message::Close(None)).await;
                return;
            }
        }
    }
    

    println!("Client connected: {} ({})", client_id, device_type);

    // Add client to peers map
    {
        let mut peers = state.peer_map.lock().await;
        peers.insert(
            client_id.clone(),
            PeerInfo {
                client_id: client_id.clone(),
                sender: tx.clone(),
                device_type: device_type.clone(),
                device: device_model.clone()
            },
        );
    }


    let server_info = app_state.get_server_data().unwrap();


    // Emit different events based on device type (mobile or desktop)
   // Construct the client message to send device and server info
    let client_message = json!({
        "action": "server_info",
        "server": {
            "ip": server_info.ip,
            "port": server_info.port,
            // "status": server_info.status,
            "device_type": server_info.device_type,
            "server_name": server_info.server_name,
        }
    });

    println!("device type: {}", device_type);
    // Emit event for mobile client
    if device_type == "mobile" {
        app_handle.emit(
            "new_mobile_peer_added",
            SerializablePeerInfo {
                client_id: client_id.to_string(),
                device_type: device_type.to_string(),
                device: device_model.to_string(),
            },
        ).unwrap_or_else(|e| eprintln!("Failed to emit new_mobile_peer_added event: {}", e));

        // Send the message to the mobile client
      
        if  write.send(Message::Text(client_message.to_string())).await.is_err() {
            eprintln!("Failed to send message to mobile client");
        } else {
            println!{"message sent to mobile client"}
        }
       
    }
   


    // Spawn a task for incoming messages
    let state_clone = state.clone();
    let client_id_clone = client_id.clone();

    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Ok(text) = msg.to_text() {
                println!("Received from {}: {}", client_id_clone, text);
                if let Ok(json) = serde_json::from_str::<Value>(text) {
                    if let Some(action) = json["action"].as_str() {
                        if action == "component" {
                            // forward_to_mobile(state_clone.clone(), json.to_string()).await;
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

    println!("Client disconnected: {}", client_id);

    // Remove client from peers map
    {
        let mut peers = state.peer_map.lock().await;
        peers.remove(&client_id);
    }
}

// async fn forward_to_mobile(state: PeerState, payload: String) {
//     let peers = state.lock().await;
//     for (client_id, peer_info) in peers.iter() {
//         if peer_info.device_type == "mobile" {
//             if let Err(e) = peer_info.sender.send(Message::Text(payload.clone())) {
//                 eprintln!("Failed to send message to {}: {}", client_id, e);
//             }
//         }
//     }
// }



fn detect_device_type(device_type: &str) -> String {
    // Log the raw input for debugging

    // Simple device type detection based on string contents
    if device_type=="smartphone" {
        "mobile".to_string()
    } else if device_type == "desktop" {
        "desktop".to_string()
    } else {
        "unknown".to_string()
    }
}



// async fn forward_to_mobile(state: PeerState, payload: String) {
//     let peers = state.lock().await;
//     for (client_id, peer) in peers.iter() {
//         if peer.device_type == "mobile" {
//             let _ = peer.sender.send(Message::Text(payload.clone()));
//         }
//     }
// }