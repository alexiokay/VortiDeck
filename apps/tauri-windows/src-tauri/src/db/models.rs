use diesel::prelude::*;
use crate::db::schema::peers;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::peers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Peer {
    pub id: Option<i32>,  // Nullable column
    pub client_id: String,
    pub ip: String,
    pub device_type: String,
    pub device: String,
    pub device_token: Option<String>, // New field for the device token

}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::db::schema::peers)]
pub struct NewPeer<'a> {
    pub client_id: &'a str,
    pub ip: &'a str,
    pub device_type: &'a str,
    pub device: &'a str,
    pub device_token: Option<&'a str>, // Token to insert (optional)
}