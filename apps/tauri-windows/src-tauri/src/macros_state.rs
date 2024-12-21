use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MacroScript {
    pub id: String,               // Unique identifier
    pub name: String,             // Name of the macro/script
    pub actions: Vec<String>,     // A sequence of actions to execute
    pub description: String,      // Optional description
}


pub struct MacroStore {
    pub macros: Mutex<HashMap<String, MacroScript>>, // Store by macro ID
}

impl MacroStore {
    pub fn new() -> Self {
        MacroStore {
            macros: Mutex::new(HashMap::new()),
        }
    }

    // Add a new macro to the store
    pub async fn add_macro(&self, macro_script: MacroScript) {
        let mut macros = self.macros.lock().await;
        macros.insert(macro_script.id.clone(), macro_script);
    }

    // Get a macro by ID
    pub async fn get_macro(&self, id: &str) -> Option<MacroScript> {
        let macros = self.macros.lock().await;
        macros.get(id).cloned()
    }

    // Remove a macro by ID
    pub async fn remove_macro(&self, id: &str) {
        let mut macros = self.macros.lock().await;
        macros.remove(id);
    }
}
