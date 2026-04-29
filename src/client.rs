use crate::commands;
use crate::errors::{BotError, CommandError};
use poise::serenity_prelude::{Client, ClientBuilder, GatewayIntents};
use poise::{builtins, Command, Framework, FrameworkOptions};
use reqwest::Client as HttpClient;
use songbird::SerenityInit;

pub struct Data {
    pub http_client: HttpClient,
}

#[derive(Debug)]
pub enum Error {
    Bot(BotError),
    Command(CommandError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Bot(err) => write!(f, "Bot error: {}", err),
            Error::Command(err) => write!(f, "Command error: {}", err),
        }
    }
}

impl From<BotError> for Error {
    fn from(err: BotError) -> Self {
        Error::Bot(err)
    }
}

impl From<CommandError> for Error {
    fn from(err: CommandError) -> Self {
        Error::Command(err)
    }
}

impl From<poise::serenity_prelude::prelude::SerenityError> for Error {
    fn from(err: poise::serenity_prelude::prelude::SerenityError) -> Self {
        Error::Bot(BotError::Client(err.to_string()))
    }
}

impl From<songbird::error::JoinError> for Error {
    fn from(err: songbird::error::JoinError) -> Self {
        Error::Bot(BotError::Client(err.to_string()))
    }
}

impl From<songbird::error::ControlError> for Error {
    fn from(err: songbird::error::ControlError) -> Self {
        Error::Command(CommandError::QueueError(err.to_string()))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Bot(BotError::Client(err.to_string()))
    }
}

impl From<songbird::input::AudioStreamError> for Error {
    fn from(err: songbird::input::AudioStreamError) -> Self {
        Error::Bot(BotError::Client(err.to_string()))
    }
}

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn build_client(token: String) -> Result<Client, Error> {
    let intents = GatewayIntents::non_privileged();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: get_registered_commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    http_client: HttpClient::new(),
                })
            })
        })
        .build();

    ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .map_err(|e| Error::Bot(BotError::Client(e.to_string())))
}

fn get_registered_commands() -> Vec<Command<Data, Error>> {
    vec![
        commands::ping(),
        commands::play(),
        commands::leave(),
        commands::join(),
        commands::pause(),
        commands::resume(),
        commands::stop(),
        commands::skip(),
        // other commands...
    ]
}
