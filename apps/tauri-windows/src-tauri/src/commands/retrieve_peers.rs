use tauri::AppHandle;
use serde::Serialize;
use tauri::Manager;
use tokio::sync::Mutex;
use crate::shared_state::{AppState, PeerState, SerializablePeerInfo};
use serde_json;



#[tauri::command]
pub async fn retrieve_peers(app_handle: AppHandle) -> Result<Vec<SerializablePeerInfo>, String> {
    // Access the app state
    let app_state = app_handle.state::<AppState>();
    let peer_state = app_handle.state::<PeerState>();

    // Lock the peer map to safely access it
    let peers = peer_state.peer_map.lock().await;

    // Log server data to verify if it returns Some or None
    if let Some(server_data) = app_state.get_server_data() {
        println!("Server IP: {}", server_data.ip);
    } else {
        println!("Server IP is not set.");
    }

    let server_ip = app_state.get_server_data().map(|data| data.ip);

    // Create a serializable list of PeerInfo, excluding the server peer by IP
    let serialized_peers: Vec<SerializablePeerInfo> = peers.values()
        .filter(|peer| {
            match &server_ip {
                Some(ip) => &peer.ip != ip, // Exclude server peer if IP matches
                None => true, // If no server IP, include all peers
            }
        })
        .map(|peer| SerializablePeerInfo {
            client_id: peer.client_id.clone(),
            device_type: peer.device_type.clone(),
            device: peer.device.clone(),
        })
        .collect();

    // Return the list of serialized peers directly
    Ok(serialized_peers)
}
