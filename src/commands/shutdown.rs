use poise::command;
use std::process::exit;

use crate::types::{Context, Error};

#[command(category = "admin", owners_only, slash_command, rename = "shutdown")]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.reply("🆗").await?;

    exit(0);
}
