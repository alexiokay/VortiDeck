mod commands;
pub mod shared_state;
use crate::shared_state::AppSecrets;
use tauri::{Manager, State, App, AppHandle};

#[tauri::command]
fn test_command() {
  println!("I was invoked from JavaScript!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
      .setup(|app: &mut App| {
        // Demonstrating how to access and modify state in setup
        let app_handle = app.handle();
        let secrets_state: State<AppSecrets> = app.state();



        Ok(())
    })
    

    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// lib.rs or a central module

