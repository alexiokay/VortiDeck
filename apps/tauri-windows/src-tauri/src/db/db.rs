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
use std::fs::OpenOptions;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;



pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");
/// Get the path for the SQLite database in the user's AppData directory
pub fn get_database_path() -> PathBuf {
    let app_data = data_dir().expect("Could not determine AppData directory");
    let db_path = app_data.join("VortiDeck").join("vortideck.db");
    db_path
}

/// Ensure the DATABASE_URL is written to the `.env` file if not already set


fn set_database_url_in_env() {
    let db_path = get_database_path();
    let db_url = format!("sqlite://{}", db_path.to_str().expect("Invalid DB Path").replace('\\', "\\\\"));

    // Construct the path to the `.env` file in the project root
    let env_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.env");

    // Read the contents of the `.env` file (or create it if it doesn't exist)
    let mut lines = Vec::new();
    if env_file_path.exists() {
        let env_file = OpenOptions::new()
            .read(true)
            .open(&env_file_path)
            .expect("Failed to open .env file for reading");

        let reader = BufReader::new(env_file);
        for line in reader.lines() {
            let line = line.expect("Failed to read line from .env");
            if line.starts_with("DATABASE_URL=") {
                lines.push(format!("DATABASE_URL={}", db_url)); // Replace the DATABASE_URL line
                println!("DATABASE_URL replaced in .env file at {}", env_file_path.display());
            } else {
                lines.push(line); // Keep other lines unchanged
            }
        }
    } else {
        println!(".env file does not exist in project root. Creating a new one...");
    }

    // If DATABASE_URL was not found, add it
    if !lines.iter().any(|line| line.starts_with("DATABASE_URL=")) {
        println!("DATABASE_URL not found in .env file. Adding it...");
        lines.push(format!("DATABASE_URL={}", db_url));
    }

    // Write the updated content back to the .env file
    let mut env_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // Overwrite the file content
        .open(&env_file_path)
        .expect("Failed to open .env file for writing");

    for line in lines {
        writeln!(env_file, "{}", line).expect("Failed to write line to .env file");
    }

    println!(".env file updated successfully at {}", env_file_path.display());

    // Set the DATABASE_URL environment variable
    std::env::set_var("DATABASE_URL", db_url);
    println!("DATABASE_URL environment variable set");
}


/// Initialize the database by running pending migrations
pub fn initialize_database() {
    set_database_url_in_env();
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
