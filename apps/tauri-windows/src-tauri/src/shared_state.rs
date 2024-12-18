// shared_state.rs
use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppSecrets {
    pub secret: Mutex<Option<String>>, // A single thread-safe secret
}

impl AppSecrets {
    /// Set a new secret
    pub fn set_secret(&self, secret_key: String) {
        let mut secret = self.secret.lock().unwrap();
        *secret = Some(secret_key);
    }

    /// Display the current secret (if available)
    pub fn display_secret(&self) {
        let secret = self.secret.lock().unwrap();
        if let Some(ref key) = *secret {
            println!("Current Secret: {}", key);
        } else {
            println!("No secret set.");
        }
    }

    /// Retrieve the current secret (if needed for other logic)
    pub fn get_secret(&self) -> String {
        let secret = self.secret.lock().unwrap();
        secret.clone().unwrap_or_default()
    }
    

    /// Clear the current secret
    pub fn clear_secret(&self) {
        let mut secret = self.secret.lock().unwrap();
        *secret = None;
    }
}