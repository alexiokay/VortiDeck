use std::sync::{Arc, RwLock};
use tokio::sync::Mutex as TokioMutex;
use std::net::IpAddr;
use tokio::sync::broadcast;
use std::collections::HashMap;
use tokio_tungstenite::{tungstenite::Message};
use serde::Serialize;
use tauri::{AppHandle};
use tauri::Manager;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializablePeerInfo  {
    pub client_id: String,
    pub device_type: String, // Keep only serializable fields
    pub device: String,
}


#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub client_id: String, // Add client_id here
    pub ip:  IpAddr,
    pub sender: broadcast::Sender<Message>,
    pub device_type: String, // "mobile" or "desktop"
    pub device: String,
}
// #[derive(Default)]
pub struct PeerState {
    pub peer_map: TokioMutex<HashMap<String, PeerInfo>>
}

impl Default for PeerState {
    fn default() -> Self {
        PeerState {
            peer_map: TokioMutex::new(HashMap::new()),
        }
    }
}

// impl PeerState {
//     pub async fn get_serialized_peers_excluding_server(&self, app_handle: AppHandle) -> Vec<SerializablePeerInfo> {
//         let app_state = app_handle.state::<AppState>();

//         let peers = self.peer_map.lock().await;

//         // Log server data to verify if it returns Some or None
//         if let Some(server_data) = app_state.get_server_data() {
//             println!("Server IP: {}", server_data.ip);
//         } else {
//             println!("Server IP is not set.");
//         }

//         let server_ip = app_state.get_server_data().map(|data| data.ip);

        

//         // Create a serializable list of PeerInfo, excluding the server peer by client_id
//         peers.values()
//             .filter(|peer| {
//                 match &server_ip {
//                     Some(ip) => &peer.ip != ip, // Exclude server peer if ip matches

//                     None => true, // If no server client_id, don't exclude any peers
//                 }
//             })
//             .map(|peer| SerializablePeerInfo {
//                 client_id: peer.client_id.clone(),
//                 device_type: peer.device_type.clone(),
//                 device: peer.device.clone(),
//             })
//             .collect()
//     }
// }


//////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct ServerInfo {
    pub ip: IpAddr,
    pub port: u16,
    // pub status: String,
    pub device_type: String,
    pub server_name: String,
}
impl Default for ServerInfo {
    
    fn default() -> Self {
        ServerInfo {
            ip: "127.0.0.1".parse::<IpAddr>().expect("Invalid IP address format"),  // Default to localhost IPv4 address
            port: 9001,  // Default port
            // status: "offline".to_string(),  // Default status
            device_type: "unknown".to_string(),  // Default device type
            server_name: "MyServer".to_string(),  // Default server name
        }
    }
}
 //////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct AppState {
    secret: RwLock<Option<String>>, // Thread-safe read/write lock
    server_data: RwLock<Option<ServerInfo>>,
    server_client_id: RwLock<Option<String>>,
    
}

impl AppState {

    // Set the server client ID. This will write to the RwLock.
    pub fn set_server_client_id(&self, new_server_client_id: String) {
        if let Ok(mut server_client_id) = self.server_client_id.write() {
            *server_client_id = Some(new_server_client_id);
        } else {
            eprintln!("Failed to acquire write lock while setting server_client_id");
        }
    }

    // Get the server client ID. This will read from the RwLock.
    pub fn get_server_client_id(&self) -> Option<String> {
        if let Ok(server_client_id) = self.server_client_id.read() {
            server_client_id.clone() // Return a clone of the Option<String>
        } else {
            eprintln!("Failed to acquire read lock while retrieving server_client_id");
            None
        }
    }

    // Set a new secret
    pub fn set_secret(&self, secret_key: String) {
        if let Ok(mut secret) = self.secret.write() {
            *secret = Some(secret_key);
        } else {
            eprintln!("Failed to acquire write lock while setting secret");
        }
    }

    // Display the current secret (if available)
    pub fn display_secret(&self) {
        if let Ok(secret) = self.secret.read() {
            if let Some(ref key) = *secret {
                println!("Current Secret: {}", key);
            } else {
                println!("No secret set.");
            }
        } else {
            eprintln!("Failed to acquire read lock while displaying secret");
        }
    }

    // Retrieve the current secret
    pub fn get_secret(&self) -> Option<String> {
        if let Ok(secret) = self.secret.read() {
            secret.clone() // Return a clone of the secret
        } else {
            eprintln!("Failed to acquire read lock while retrieving secret");
            None
        }
    }

    // Check if a secret exists
    pub fn has_secret(&self) -> bool {
        if let Ok(secret) = self.secret.read() {
            secret.is_some()
        } else {
            eprintln!("Failed to acquire read lock while checking secret existence");
            false
        }
    }

    // Clear the current secret
    pub fn clear_secret(&self) {
        if let Ok(mut secret) = self.secret.write() {
            *secret = None;
        } else {
            eprintln!("Failed to acquire write lock while clearing secret");
        }
    }

    // Set server data
    pub fn set_server_data(&self, ip: IpAddr, port: u16, device_type: String, server_name: String) { //status: String,
        let server_data = ServerInfo {
            ip,
            port,
            // status,
            device_type,
            server_name,
        };
        if let Ok(mut write_lock) = self.server_data.write() {
            *write_lock = Some(server_data);
        }
    }

    // Retrieve server data
    pub fn get_server_data(&self) -> Option<ServerInfo> {
        if let Ok(read_lock) = self.server_data.read() {
            // Clone the data from the RwLockReadGuard
            read_lock.clone() // Dereference and clone the contained data
        } else {
            None
        }
    }
}