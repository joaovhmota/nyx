use dotenv::dotenv;
use logfy::{critical, error, information, warning};
use poise::CreateReply;
use poise::serenity_prelude::{Color, CreateEmbed, Timestamp};
use poise::{Command, FrameworkError, FrameworkOptions};
use serenity::all::{Http, UserId};
use std::{collections::HashSet, env, process::exit, vec};

use crate::{
    commands::{about_user::about_user, profile_picture::profile_picture, roll_dice::roll_dice},
    types::{Context, Error},
    utils::user_utils::get_user_name,
};

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

pub fn get_commands() -> Vec<Command<(), Error>> {
    vec![profile_picture(), about_user(), roll_dice()]
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
}

pub async fn on_error(error: FrameworkError<'_, (), Error>) {
    let (embed, ctx) = match error {
        FrameworkError::NotAnOwner { ctx, .. } => {
            warning!("Unauthorized access attempt by {}", ctx.author().id);

            let embed = CreateReply::default().embed(
                CreateEmbed::default()
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
                    .title("❌ Command failed")
                    .description(format!("{error}"))
                    .color(Color::RED)
                    .timestamp(Timestamp::now()),
            );

            (embed, ctx)
        }
        _ => todo!("Error"),
    };

    let _ = ctx.send(embed.ephemeral(true)).await;
}

pub async fn get_framework_options(token: &str) -> FrameworkOptions<(), Error> {
    FrameworkOptions {
        commands: get_commands(),
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
