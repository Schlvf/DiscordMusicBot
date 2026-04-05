use crate::client::{Context, Error};
use poise::serenity_prelude::GuildId;
use songbird::input::{Compose, YoutubeDl};
use songbird::{Call, Songbird};
use std::sync::Arc;

// ---------- Helpers ----------

async fn get_manager(ctx: &Context<'_>) -> Result<Arc<Songbird>, Error> {
    songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| "Songbird not initialized".into())
}

fn get_guild_id(ctx: &Context<'_>) -> Result<GuildId, Error> {
    ctx.guild_id().ok_or_else(|| "Not in a guild".into())
}

async fn get_call(
    manager: &Songbird,
    guild_id: GuildId,
) -> Result<std::sync::Arc<tokio::sync::Mutex<Call>>, Error> {
    manager
        .get(guild_id)
        .ok_or_else(|| -> Error { "Not connected to a voice channel".into() })
}

// Small helper to convert errors into user messages
macro_rules! user_error {
    ($ctx:expr, $expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                $ctx.say(err.to_string()).await?;
                return Ok(());
            }
        }
    };
}

// ---------- Commands ----------

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    ctx.say("pong").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = user_error!(ctx, get_guild_id(&ctx));

    let channel_id = match ctx.guild().and_then(|g| {
        g.voice_states
            .get(&ctx.author().id)
            .and_then(|vs| vs.channel_id)
    }) {
        Some(channel) => channel,
        None => {
            ctx.say("You must be in a voice channel").await?;
            return Ok(());
        }
    };

    let manager = user_error!(ctx, get_manager(&ctx).await);

    if let Err(err) = manager.join(guild_id, channel_id).await {
        ctx.say("Failed to join voice channel").await?;
        eprintln!("Join error: {:?}", err);
    } else {
        ctx.say("Joined voice channel").await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = user_error!(ctx, get_guild_id(&ctx));
    let manager = user_error!(ctx, get_manager(&ctx).await);

    if manager.get(guild_id).is_none() {
        ctx.say("Not connected").await?;
        return Ok(());
    }

    manager.remove(guild_id).await?;
    ctx.say("Left voice channel").await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "URL of the source"] url: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !url.starts_with("http") {
        ctx.say("Invalid URL").await?;
        return Ok(());
    }

    let guild_id = user_error!(ctx, get_guild_id(&ctx));
    let manager = user_error!(ctx, get_manager(&ctx).await);
    let call_mutex = get_call(&manager, guild_id).await?;
    let mut call = call_mutex.lock().await;

    let client = &ctx.data().http_client;

    let mut src = YoutubeDl::new(client.clone(), url.clone())
        .user_args(vec!["-f".into(), "--no-playlist".into()]);

    let metadata = src.aux_metadata().await?;

    call.enqueue_input(src.into()).await.set_volume(0.1)?;

    let title = metadata.title.unwrap_or_else(|| "Unknown".to_string());
    let position = call.queue().len();

    ctx.say(format!(
        "Added '{}' to queue at position {}",
        title, position
    ))
    .await?;

    Ok(())
}

// ---------- Queue Actions ----------

async fn handle_queue_action<F>(ctx: Context<'_>, action: F, success_msg: &str) -> Result<(), Error>
where
    F: FnOnce(&Call) -> Result<(), songbird::error::ControlError>,
{
    let guild_id = user_error!(ctx, get_guild_id(&ctx));
    let manager = user_error!(ctx, get_manager(&ctx).await);
    let call_mutex = get_call(&manager, guild_id).await?;
    let call = call_mutex.lock().await;

    if let Err(err) = action(&call) {
        ctx.say("Queue operation failed").await?;
        eprintln!("Queue error: {:?}", err);
        return Ok(());
    }

    ctx.say(success_msg).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    handle_queue_action(ctx, |c| c.queue().pause(), "Paused").await
}

#[poise::command(slash_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    handle_queue_action(ctx, |c| c.queue().resume(), "Resumed").await
}

#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    handle_queue_action(ctx, |c| c.queue().skip(), "Skipped").await
}

#[poise::command(slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = user_error!(ctx, get_guild_id(&ctx));
    let manager = user_error!(ctx, get_manager(&ctx).await);
    let call_mutex = get_call(&manager, guild_id).await?;
    let call = call_mutex.lock().await;

    call.queue().stop();

    ctx.say("Stopped playback").await?;
    Ok(())
}
