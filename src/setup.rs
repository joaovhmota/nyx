use dotenv::dotenv;
use logfy::{critical, debug, error, information};
use poise::{Command, FrameworkError, FrameworkOptions};
use serenity::all::{Http, UserId};
use std::{collections::HashSet, env, process::exit, vec};

use crate::{
    commands::{die::die, ping::ping},
    types::{Context, Error},
    utils::user::get_user_name,
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
    vec![ping(), die()]
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
    match error {
        FrameworkError::Command {
            error: err, ctx, ..
        } => {
            error!("Command '{}' failed: {:?}", ctx.command().name, err);

            let _ = ctx.defer_ephemeral().await;
            let _ = ctx.reply("❌ Algo deu errado.").await;
        }
        FrameworkError::NotAnOwner { ctx, .. } => {
            debug!("Unauthorized access attempt by {}", ctx.author().id);

            let _ = ctx.defer_ephemeral().await;
            let _ = ctx.reply("🚫 Você não tem acesso a esse comando.").await;
        }
        other => {
            error!("Unhandled error: {:?}", other);
        }
    }
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
