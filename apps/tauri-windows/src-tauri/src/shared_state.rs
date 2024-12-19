use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct AppSecrets {
    secret: RwLock<Option<String>>, // Thread-safe read/write lock
}

impl AppSecrets {
    /// Set a new secret
    pub fn set_secret(&self, secret_key: String) {
        if let Ok(mut secret) = self.secret.write() {
            *secret = Some(secret_key);
        } else {
            eprintln!("Failed to acquire write lock while setting secret");
        }
    }

    /// Display the current secret (if available)
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

    /// Retrieve the current secret
    pub fn get_secret(&self) -> Option<String> {
        if let Ok(secret) = self.secret.read() {
            secret.clone() // Return a clone of the secret
        } else {
            eprintln!("Failed to acquire read lock while retrieving secret");
            None
        }
    }

    /// Check if a secret exists
    pub fn has_secret(&self) -> bool {
        if let Ok(secret) = self.secret.read() {
            secret.is_some()
        } else {
            eprintln!("Failed to acquire read lock while checking secret existence");
            false
        }
    }

    /// Clear the current secret
    pub fn clear_secret(&self) {
        if let Ok(mut secret) = self.secret.write() {
            *secret = None;
        } else {
            eprintln!("Failed to acquire write lock while clearing secret");
        }
    }
}
