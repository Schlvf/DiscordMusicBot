use crate::commands;
use crate::errors::BotError;
use poise::serenity_prelude::{Client, ClientBuilder, GatewayIntents};
use poise::{builtins, Command, Framework, FrameworkOptions};
use reqwest::Client as HttpClient;
use songbird::SerenityInit;

pub struct Data {
    pub http_client: HttpClient,
}

pub type Error = BotError;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn build_client(token: String) -> Result<Client, BotError> {
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
        .map_err(|e| BotError::Client(e.to_string()))
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
