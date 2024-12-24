// src/commands/mod.rs
pub mod db;
pub mod models;
pub mod schema;
pub mod utils;

pub use db::initialize_database;  // This line simplifies access
pub use models::*;
pub use schema::*;
pub use utils::*;