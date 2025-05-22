mod client;
mod commands;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    setup();

    let discord_client = client::build_client().await;
    discord_client.unwrap().start().await.unwrap();
}

fn setup() {
    match dotenv() {
        Ok(_) => println!("Loaded .env file successfully"),
        Err(err) => eprintln!("Warning: Could not load .env file: {}", err),
    }
}
