use songbird::input::AudioStreamError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BotError {
    Environment(String),
    Dependency(String),
    Client(String),
    Runtime(String),
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BotError::Environment(msg) => write!(f, "Environment error: {}", msg),
            BotError::Dependency(msg) => write!(f, "Dependency error: {}", msg),
            BotError::Client(msg) => write!(f, "Client error: {}", msg),
            BotError::Runtime(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl Error for BotError {}

impl From<poise::serenity_prelude::prelude::SerenityError> for BotError {
    fn from(err: poise::serenity_prelude::prelude::SerenityError) -> Self {
        BotError::Client(err.to_string())
    }
}

impl From<songbird::error::JoinError> for BotError {
    fn from(err: songbird::error::JoinError) -> Self {
        BotError::Client(err.to_string())
    }
}

impl From<AudioStreamError> for BotError {
    fn from(err: AudioStreamError) -> Self {
        BotError::Client(err.to_string())
    }
}

impl From<songbird::error::ControlError> for BotError {
    fn from(err: songbird::error::ControlError) -> Self {
        BotError::Client(err.to_string())
    }
}

impl From<CommandError> for BotError {
    fn from(err: CommandError) -> Self {
        BotError::Runtime(err.to_string())
    }
}

// ---------- Command Errors ----------

#[derive(Debug)]
pub enum CommandError {
    NotInGuild,
    NotConnected,
    NotInVoiceChannel,
    SongbirdNotInitialized,
    InvalidUrl,
    JoinFailed,
    QueueError(String),
    DownloadError(String),
    Runtime(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::NotInGuild => write!(f, "Not in a guild"),
            CommandError::NotConnected => write!(f, "Not connected to a voice channel"),
            CommandError::NotInVoiceChannel => write!(f, "You must be in a voice channel"),
            CommandError::SongbirdNotInitialized => write!(f, "Songbird not initialized"),
            CommandError::InvalidUrl => write!(f, "Invalid URL"),
            CommandError::JoinFailed => write!(f, "Failed to join voice channel"),
            CommandError::QueueError(msg) => write!(f, "Queue operation failed: {}", msg),
            CommandError::DownloadError(msg) => write!(f, "Failed to download stream: {}", msg),
            CommandError::Runtime(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl Error for CommandError {}

impl From<songbird::error::ControlError> for CommandError {
    fn from(err: songbird::error::ControlError) -> Self {
        CommandError::QueueError(err.to_string())
    }
}

impl From<reqwest::Error> for CommandError {
    fn from(err: reqwest::Error) -> Self {
        CommandError::DownloadError(err.to_string())
    }
}
