use crate::client::{Context, Error};
use reqwest::Client;
use songbird::input::{Compose, YoutubeDl};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "YouTube URL or search term"] query: String,
) -> Result<(), Error> {
    if !&query.starts_with("http") {
        ctx.say("Missing valid url").await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird not initialized")?
        .clone();

    if let Some(call_mutex) = manager.get(guild_id) {
        ctx.say("Request received").await?;

        let mut call = call_mutex.lock().await;
        let mut src = YoutubeDl::new(Client::new(), query.clone()).user_args(vec![
            "-f".into(),
            "--no-playlist".into(), // skip playlist-expansion
        ]);

        let metadata = src.aux_metadata().await?;

        call.enqueue_input(src.into()).await.set_volume(0.1)?;

        let output = format!(
            "Added to the queue '{}' in position {}",
            metadata.title.as_ref().unwrap_or(&"Unknown".to_string()),
            call.queue().len()
        );
        ctx.channel_id().say(&ctx.http(), output).await?;
    } else {
        ctx.say("Not in a voice channel to play in").await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let channel_id = ctx
        .guild()
        .ok_or("Unable to find guild for this context")?
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or("You must be in a voice channel")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird not initialized")?
        .clone();

    // Join the VC (or move)
    match manager.join(guild_id, channel_id).await {
        Ok(_) => {
            ctx.say("Joined voice channel").await?;
        }
        Err(err) => {
            ctx.say("Failed to join voice channel").await?;
            eprintln!("Join error: {:?}", err);
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird not initialized")?
        .clone();

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
        ctx.say("Left voice channel").await?;
    } else {
        ctx.say("Not connected").await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird not initialized")?
        .clone();

    if let Some(call_mutex) = manager.get(guild_id) {
        let call = call_mutex.lock().await;

        match call.queue().pause() {
            Ok(_) => {
                ctx.say("Paused the player").await?;
            }
            Err(err) => {
                ctx.say("Unable to pause the player").await?;
                eprintln!("Pause error: {:?}", err);
            }
        };
    } else {
        ctx.say("Not connected").await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird not initialized")?
        .clone();

    if let Some(call_mutex) = manager.get(guild_id) {
        let call = call_mutex.lock().await;

        match call.queue().resume() {
            Ok(_) => {
                ctx.say("Resumed the player").await?;
            }
            Err(err) => {
                ctx.say("Unable to resume the player").await?;
                eprintln!("Resume error: {:?}", err);
            }
        };
    } else {
        ctx.say("Not connected").await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird not initialized")?
        .clone();

    if let Some(call_mutex) = manager.get(guild_id) {
        let call = call_mutex.lock().await;

        match call.queue().skip() {
            Ok(_) => {
                ctx.say("Skipping the current track").await?;
            }
            Err(err) => {
                ctx.say("Unable to skip the current track").await?;
                eprintln!("Skip error: {:?}", err);
            }
        };
    } else {
        ctx.say("Not connected").await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird not initialized")?
        .clone();

    if let Some(call_mutex) = manager.get(guild_id) {
        let call = call_mutex.lock().await;
        call.queue().stop();
        ctx.say("Stopping the current session").await?;
    } else {
        ctx.say("Not connected").await?;
    }

    Ok(())
}
