mod client;
mod commands;

use dotenvy::dotenv;
use which::which;

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

    for dep in &["yt-dlp", "ffmpeg"] {
        which(dep).unwrap_or_else(|_| {
            eprintln!("Error: `{}` not found in PATH", dep);
            std::process::exit(1);
        });
    }
    println!("All dependencies found");
}
