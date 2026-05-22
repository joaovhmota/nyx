use crate::db::mongodb::NyxMongo;
use crate::types::{Context, Error};
use crate::utils::embed_builder_helper::EmbedBuilderHelper;
use futures::stream::StreamExt;
use mongodb::bson::{Document, doc};
use poise::command;
use std::cmp::Reverse;

/// Shows a summary of Nyx's usage, showing the top 25 users
#[command(slash_command, owners_only)]
pub async fn usage(ctx: Context<'_>) -> Result<(), Error> {
    let db = NyxMongo::get_db().await?;
    let collection = db.collection::<Document>("users");

    let mut cursor = collection.find(doc! {}).await?;

    let mut users_data = Vec::new();
    let mut total_commands: f64 = 0.0;

    while let Some(result) = cursor.next().await {
        if let Ok(doc) = result
            && let Ok(id) = doc.get_str("_id")
        {
            let executed = doc
                .get_i64("commands_executed")
                .or_else(|_| doc.get_i32("commands_executed").map(|i| i as i64))
                .unwrap_or(0);

            if executed > 0 {
                users_data.push((id.to_string(), executed));
                total_commands += executed as f64;
            }
        }
    }

    users_data.sort_by_key(|b| Reverse(b.1));

    let context_cache = ctx.cache();

    let mut embed = EmbedBuilderHelper::new(ctx)
        .await?
        .with_title("📊 Nyx's Usage");

    if let Some(url) = context_cache.current_user().avatar_url() {
        embed = embed.with_thumbnail(url)
    }

    if users_data.is_empty() {
        embed = embed.with_description("Nothing to see here.");
    } else {
        embed = embed
            .with_description("Summary containing all command requests/executions and the top 25 users with the most requests/executions.")
            .with_field(
                "Amount of commands requested/executed",
                format!("{total_commands}"),
                false,
            );

        users_data.truncate(25);

        for (id, executed) in users_data {
            let percentage = if total_commands > 0.0 {
                (executed as f64 / total_commands) * 100.0
            } else {
                0.0
            };

            embed = embed
                .with_field("User", format!("<@{id}>"), true)
                .with_field("# of commands", format!("{executed}\n"), true)
                .with_field("% of executions", format!("{percentage:.2}%\n"), true)
                .with_field(String::default(), String::default(), false);
        }
    }

    ctx.send(embed.into()).await?;

    Ok(())
}
