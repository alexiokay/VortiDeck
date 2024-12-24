use std::fs;
use std::path::PathBuf;
use dirs::data_dir;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;



pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");
/// Get the path for the SQLite database in the user's AppData directory
pub fn get_database_path() -> PathBuf {
    let app_data = data_dir().expect("Could not determine AppData directory");
    let db_path = app_data.join("VortiDeck").join("vortideck.db");
    db_path
}

/// Initialize the database by running pending migrations
pub fn initialize_database() {
    let db_path = get_database_path();
    println!("initialising database");
    // Ensure the database directory exists
    if let Some(parent_dir) = db_path.parent() {
        fs::create_dir_all(parent_dir).expect("Failed to create database directory");
        println!("creted directory")
    }

    // Set the DATABASE_URL environment variable for Diesel
    std::env::set_var("DATABASE_URL", db_path.to_str().expect("Invalid database path"));

    // Run migrations
    let mut connection = establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

/// Establish a connection to the SQLite database
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Initialize the connection pool
pub fn establish_pool() -> DbPool {
    dotenv().ok(); // Load environment variables from `.env`
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")

    // let pool = Pool::builder()
    //     .max_size(16) /1
    //     .connection_customizer(Box::new(ConnectionOptions {
    //         enable_wal: true,
    //         enable_foreign_keys: true,
    //         busy_timeout: Some(Duration::from_secs(30)),
    //     }))
    //     .build(ConnectionManager::<SqliteConnection>::new(database_url))
    //     .unwrap();
    //? SRC: https://stackoverflow.com/questions/57123453/how-to-use-diesel-with-sqlite-connections-and-avoid-database-is-locked-type-of

}

/// Helper function to get a connection from the pool
pub fn get_connection(pool: &DbPool) -> PooledConnection<ConnectionManager<SqliteConnection>> {
    match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to get a database connection: {}", err);
            panic!("Database connection error"); // Optionally panic or return a `Result`
        }
    }
}
