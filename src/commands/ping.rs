use crate::types::{Context, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.reply("🏓 Pong!").await?;

    Ok(())
}
