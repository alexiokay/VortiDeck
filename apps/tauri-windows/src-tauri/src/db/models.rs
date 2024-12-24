use diesel::prelude::*;
use crate::db::schema::peers;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::peers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PeerModel {
    pub id: Option<i32>,  // Nullable column
    pub client_id: String,
    pub ip: String,
    pub device_type: String,
    pub device: String,
}
