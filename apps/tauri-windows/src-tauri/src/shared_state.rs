use std::sync::{Arc, RwLock};
use std::net::IpAddr;

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
            ip: IpAddr::V4("127.0.0.1".parse().unwrap()),  // Default to localhost IPv4 address
            port: 9001,  // Default port
            // status: "offline".to_string(),  // Default status
            device_type: "unknown".to_string(),  // Default device type
            server_name: "MyServer".to_string(),  // Default server name
        }
    }
}

#[derive(Default)]
pub struct AppState {
    secret: RwLock<Option<String>>, // Thread-safe read/write lock
    server_data: RwLock<Option<ServerInfo>>,
}

impl AppState {
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