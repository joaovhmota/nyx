use crate::types::{Context, Error};
use serenity::all::User;

pub fn name_ends_with_s(user: &User) -> bool {
    user.display_name()
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
    let user_id = if let Some(informed_user) = &user {
        informed_user.id
    } else {
        ctx.author().id
    };

    let target_user = ctx.http().get_user(user_id).await?;

    if ctx.author().id == target_user.id && set_ephemeral {
        ctx.defer_ephemeral().await?;
    } else {
        ctx.defer().await?;
    }

    Ok(target_user)
}
