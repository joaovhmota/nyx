use poise::command;
use std::process::exit;

use crate::types::{Context, Error};

#[command(slash_command, owners_only)]
pub async fn die(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.reply("Indo offline…").await?;
    exit(0);
}
