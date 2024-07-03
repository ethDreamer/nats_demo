use nats_demo::{
    types::BlockSummary,
    models::BlockSummaryEntry,
};
use nats::connect;
use ssz::Decode;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::mpsc;
use std::thread;
use nats_demo::schema::block_summaries;

extern crate diesel;

// Embed the migrations into the binary
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the subject and NATS server address from environment variables
    let subject = env::var("NATS_SUBJECT").unwrap_or_else(|_| "example.subject".to_string());
    let nats_server = env::var("NATS_SERVER").unwrap_or_else(|_| "nats://nats-service:4222".to_string());
    println!("Subscribing to subject: {}", subject);
    println!("Connecting to NATS server: {}", nats_server);

    // Connect to the NATS server
    let nc = connect(&nats_server)?;

    // Read PostgreSQL connection details from environment variables
    let postgres_user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let postgres_password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "mysecretpassword".to_string());
    let postgres_db = env::var("POSTGRES_DB").unwrap_or_else(|_| "mydb".to_string());
    let postgres_host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "postgres".to_string());
    let postgres_url = format!("postgres://{}:{}@{}/{}", postgres_user, postgres_password, postgres_host, postgres_db);
    let mut connection = PgConnection::establish(&postgres_url)?;

    // Run the migrations
    connection.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

    // Create a channel with explicit types
    let (tx, rx): (mpsc::Sender<(_, Vec<u8>)>, mpsc::Receiver<(_, Vec<u8>)>) = mpsc::channel();


    // Spawn a thread for database insertion
    let postgres_url_clone = postgres_url.clone();
    thread::spawn(move || {
        let mut conn = PgConnection::establish(&postgres_url_clone).expect("Failed to connect to the database");
        while let Ok((timestamp, data)) = rx.recv() {
            let block_summary = BlockSummary::from_ssz_bytes(&data)
                .expect("Failed to decode bytes");

            println!("Inserting block summary: {:?}", block_summary);
            let entry = BlockSummaryEntry::from_components(block_summary, timestamp);
            insert_block_summary(&mut conn, &entry).expect("Failed to insert block summary");
        }
    });

    // Subscribe to the subject
    let sub = nc.subscribe(&subject)?;

    for msg in sub.messages() {
        let timestamp = chrono::Utc::now().naive_utc(); // Get the current timestamp
        tx.send((timestamp, msg.data.to_vec())).expect("Failed to send data to the channel");
    }

    Ok(())
}

fn insert_block_summary(conn: &mut PgConnection, entry: &BlockSummaryEntry) -> Result<(), diesel::result::Error> {
    diesel::insert_into(block_summaries::table)
        .values(entry)
        .execute(conn)?;

    Ok(())
}
