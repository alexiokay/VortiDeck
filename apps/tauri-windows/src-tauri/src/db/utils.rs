use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::db::schema::peers::dsl::*;
use crate::db::schema::peers;
use crate::db::models::*;
use tauri::AppHandle;
use crate::shared_state::SerializablePeerInfo;
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
        peers
            .select(diesel::dsl::count_star())
            .first(&mut conn)
            .expect("Error counting peers")
    }

    // New method to add a peer to the database
    pub fn add_peer<'a>(&self, new_peer: NewPeer<'a>) -> Result<(), diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        // Insert the new peer into the database
        diesel::insert_into(peers::table)
            .values(&new_peer)
            .execute(&mut conn)?;

        Ok(())
    }

    pub async fn get_all_peers(&self) -> Vec<SerializablePeerInfo> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result = peers
            .load::<Peer>(&mut conn)
            .expect("Error loading peers");

        result.into_iter().map(|peer| SerializablePeerInfo {
            client_id: peer.client_id,
            device_type: peer.device_type,
            device: peer.device,
        }).collect()
    }

    pub fn get_peer_by_token(&self, token: &str) -> Option<Peer> {


        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result = peers
            .filter(device_token.eq(token))  // Filter by the device_token column
            .first::<Peer>(&mut conn)
            .optional()  // Return Option<Peer>
            .expect("Error querying peer by token");

        result
    }

   




}