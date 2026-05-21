use crate::{
    commands::avaliable_commands,
    db::mongodb::NyxMongo,
    types::{Context, Error},
    utils::user_utils::get_user_name,
};
use bson::Document;
use dotenv::dotenv;
use logfy::{critical, error, information, warning};
use mongodb::bson::doc;
use poise::CreateReply;
use poise::serenity_prelude::{Color, CreateEmbed, Timestamp};
use poise::{FrameworkError, FrameworkOptions};
use serenity::{
    all::{Http, UserId},
    builder::CreateEmbedAuthor,
};
use std::{collections::HashSet, env, process::exit};

pub fn get_token() -> String {
    if cfg!(debug_assertions) {
        match dotenv() {
            Ok(env_path) => {
                information!("Successfully read varaibles from '{}'", env_path.display())
            }
            Err(err) => {
                critical!("Could not read variables from .env files. Reason: {}", err);
                exit(1)
            }
        };
    }

    match env::var("TOKEN") {
        Ok(token_value) => token_value,
        Err(err) => {
            critical!(
                "Could not get 'TOKEN' variable from environment. Reason: {}",
                err
            );
            exit(1);
        }
    }
}

pub async fn get_owner(token: &str) -> HashSet<UserId> {
    let mut owners = HashSet::new();
    let http = Http::new(token);

    if let Ok(app_info) = http.get_current_application_info().await
        && let Some(owner) = app_info.owner
    {
        information!("Application Owner is '{}'", get_user_name(&owner));
        owners.insert(owner.id);
    }

    owners
}

pub async fn on_pre_command(ctx: &Context<'_>) {
    let user = &ctx.author();
    let command = &ctx.command();

    information!(
        "User '{}' ({}) requested the execution of the command '{}'",
        get_user_name(user),
        user.id,
        command.name
    );

    match NyxMongo::get_db().await {
        Ok(db) => {
            let collection = db.collection::<Document>("users");

            let filter = doc! { "_id": user.id.to_string() };
            let update = doc! { "$inc": { "commands_executed": 1 } };
            let options = mongodb::options::UpdateOptions::builder()
                .upsert(true)
                .build();

            if let Err(err) = collection
                .update_one(filter, update)
                .with_options(options)
                .await
            {
                error!(
                    "Failed to update command execution count in MongoDB: {}",
                    err
                );
            }
        }
        Err(err) => {
            critical!("Failed to connect to MongoDB: {}", err);
        }
    }
}

pub async fn on_error(error: FrameworkError<'_, (), Error>) {
    let (embed, ctx) = match error {
        FrameworkError::NotAnOwner { ctx, .. } => {
            warning!("Unauthorized access attempt by {}", ctx.author().id);

            let embed = CreateReply::default().embed(
                CreateEmbed::default()
                    .author(
                        CreateEmbedAuthor::new(ctx.author().display_name().to_string())
                            .icon_url(ctx.author().avatar_url().unwrap_or_default()),
                    )
                    .title("🚫 Unauthorized")
                    .description("You're not allowed to use this command.")
                    .color(Color::ORANGE)
                    .timestamp(Timestamp::now()),
            );

            (embed, ctx)
        }
        FrameworkError::Command { error, ctx, .. } => {
            error!("Command failed: {:?}", error);

            let embed = CreateReply::default().embed(
                CreateEmbed::default()
                    .author(
                        CreateEmbedAuthor::new(ctx.author().display_name().to_string())
                            .icon_url(ctx.author().avatar_url().unwrap_or_default()),
                    )
                    .title("❌ Command failed")
                    .description(format!("{error}"))
                    .color(Color::RED)
                    .timestamp(Timestamp::now()),
            );

            (embed, ctx)
        }
        _ => todo!("Error"),
    };

    let _ = ctx.defer_ephemeral().await;
    let _ = ctx.send(embed.ephemeral(true)).await;
}

pub async fn get_framework_options(token: &str) -> FrameworkOptions<(), Error> {
    FrameworkOptions {
        commands: avaliable_commands(),
        owners: get_owner(token).await,
        pre_command: |ctx| {
            Box::pin(async move {
                on_pre_command(&ctx).await;
            })
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    }
}
