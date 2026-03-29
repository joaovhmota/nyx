use serenity::all::User;

use crate::types::{Context, Error};

pub fn get_user_name(user: &User) -> &String {
    match &user.global_name {
        Some(user_global_name) => user_global_name,
        None => &user.name,
    }
}

pub fn name_ends_with_s(user: &User) -> bool {
    get_user_name(user)
        .chars()
        .last()
        .map(|c| c.to_ascii_lowercase())
        == Some('s')
}

pub fn get_possession_suffix(user: &User) -> &str {
    if name_ends_with_s(user) { "'" } else { "'s" }
}

pub async fn get_target_user(
    ctx: &Context<'_>,
    user: &Option<User>,
    set_ephemeral: bool,
) -> Result<User, Error> {
    let target_user = user.clone().unwrap_or(ctx.author().clone());

    if ctx.author().id == target_user.id && set_ephemeral {
        ctx.defer_ephemeral().await?;
    } else {
        ctx.defer().await?;
    }

    Ok(target_user)
}
