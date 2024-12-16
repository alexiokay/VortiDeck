// src/commands/mod.rs
pub mod discover_websocket;
pub mod generate_qr_code;

pub use discover_websocket::discover_websocket;  // This line simplifies access
pub use generate_qr_code::generate_qr_code;  // This line simplifies access