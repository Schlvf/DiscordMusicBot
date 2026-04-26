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
