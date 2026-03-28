pub mod commands;
pub mod setup;
pub mod types;
pub mod utils;

use crate::setup::{get_framework_options, get_token};
use logfy::{critical, information, success};
use poise::{Framework, samples::register_globally};
use serenity::all::{ClientBuilder, GatewayIntents};
use std::process::exit;

#[tokio::main]
async fn main() {
    information!("Starting…");

    let token = get_token();
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS;

    let framework = Framework::builder()
        .options(get_framework_options(&token).await)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                register_globally(ctx, &framework.options().commands).await?;

                Ok(())
            })
        })
        .build();

    let mut client = match ClientBuilder::new(token, intents)
        .framework(framework)
        .await
    {
        Ok(client_value) => {
            success!("Successfully created client");
            client_value
        }
        Err(err) => {
            critical!("Could not create client. Reason: {}", err);
            exit(1);
        }
    };

    match client.start().await {
        Ok(_) => {
            information!("Disposing…");
        }
        Err(why) => {
            critical!("Could not start client: {:?}", why);
            exit(1);
        }
    }
}
