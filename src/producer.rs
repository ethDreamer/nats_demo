use nats_demo::types::BlockSummary;
use nats::connect;
use ssz::Encode;
use std::{env, time::Duration, thread};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the subject and NATS server address from environment variables
    let subject = env::var("NATS_SUBJECT").unwrap_or_else(|_| "example.subject".to_string());
    let nats_server = env::var("NATS_SERVER").unwrap_or_else(|_| "nats://nats-service:4222".to_string());
    println!("Publishing to subject: {}", subject);
    println!("Connecting to NATS server: {}", nats_server);

    // Connect to the NATS server
    let nc = connect(&nats_server)?;

    loop {
        // Create a BlockSummary instance
        let block_summary = BlockSummary::random();

        // Publish the BlockSummary instance to the subject
        nc.publish(&subject, block_summary.as_ssz_bytes())?;
        println!("Published message to subject: {:?}", block_summary);

        // Sleep for 1 second
        thread::sleep(Duration::from_secs(5));
    }
}
