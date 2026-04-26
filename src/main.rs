mod client;
mod commands;
mod config;
mod errors;

use dotenvy::dotenv;
use errors::BotError;
use std::io;
use which::which;

const DEPENDENCIES: &[&str] = &["yt-dlp", "ffmpeg"];

const DISCORD_TOKEN_ENV: &str = "DISCORD_TOKEN";

fn press_any_key_to_exit() {
    let mut _dummy = String::new();
    eprintln!("Press any key to exit...");
    io::stdin().read_line(&mut _dummy).unwrap();
}

pub fn verify_dependencies() -> Result<(), BotError> {
    for dep in DEPENDENCIES {
        match which(dep) {
            Ok(_) => continue,
            Err(_) => {
                return Err(BotError::Dependency(format!("Missing dependency: {}", dep)));
            }
        }
    }
    Ok(())
}

pub fn load_environment() -> Result<(), BotError> {
    dotenv().map_err(|e| BotError::Environment(format!("Failed to load .env file: {}", e)))?;
    Ok(())
}

async fn run() -> Result<(), BotError> {
    load_environment()?;
    verify_dependencies()?;

    let token = config::load_string(DISCORD_TOKEN_ENV)?;
    let mut discord_client = client::build_client(token).await?;
    discord_client.start().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Bot startup failed: {}", e);
        press_any_key_to_exit();
        std::process::exit(1);
    }
}
