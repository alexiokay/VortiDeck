use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::db::schema::peers::dsl::peers as peers_table;
use crate::db::models::*;
use tauri::AppHandle;

pub struct Database {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    pub fn new(pool: &Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Database {
            pool: pool.clone(), // Clone the pool here
        }
    }

    pub fn get_peer_count(&self) -> i64 {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        peers_table
            .select(diesel::dsl::count_star())
            .first(&mut conn)
            .expect("Error counting peers")
    }
}