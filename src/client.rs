use crate::commands;

use poise::serenity_prelude::prelude::SerenityError;
use poise::serenity_prelude::{Client, ClientBuilder, GatewayIntents};
use poise::{builtins, Command, Framework, FrameworkOptions};
use songbird::SerenityInit;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn build_client() -> Result<Client, SerenityError> {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = GatewayIntents::non_privileged();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: get_registered_commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await
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
