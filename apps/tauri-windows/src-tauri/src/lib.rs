
mod commands;


pub mod shared_state;
pub mod db;

use crate::shared_state::AppState;
use tauri::{Manager, State, App, AppHandle};

#[tauri::command]
fn test_command() {
  println!("I was invoked from JavaScript!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  
  tauri::Builder::default()
    
    
    
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// lib.rs or a central module

