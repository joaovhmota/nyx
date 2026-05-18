use std::cmp::Reverse;

use crate::db::mongodb::NyxMongo;
use crate::types::{Context, Error};
use futures::stream::StreamExt;
use mongodb::bson::{Document, doc};
use poise::serenity_prelude::{Color, CreateEmbed, Timestamp};
use poise::{CreateReply, command};
use serenity::builder::CreateEmbedAuthor;

#[command(slash_command)]
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

    let mut embed = CreateEmbed::default()
        .title("📊 Nyx's Usage")
        .color(Color::DARK_PURPLE)
        .author(
            CreateEmbedAuthor::new(ctx.author().display_name().to_string())
                .icon_url(ctx.author().avatar_url().unwrap_or_default()),
        )
        .timestamp(Timestamp::now());

    if users_data.is_empty() {
        embed = embed.description("Nothing to see here.");
    } else {
        embed = embed.description(format!("Amount of commands executed: `{total_commands}`"));

        users_data.truncate(25);

        for (id, executed) in users_data {
            let percentage = if total_commands > 0.0 {
                (executed as f64 / total_commands) * 100.0
            } else {
                0.0
            };

            embed = embed.field("User", format!("<@{id}>"), true);
            embed = embed.field("# of commands", format!("**{executed}**"), true);
            embed = embed.field("% of executions", format!("**{percentage:.2}%**"), true);
            embed = embed.field("", "", false);
        }
    }

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
