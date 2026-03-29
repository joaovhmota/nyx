use crate::{
    types::{Context, Error},
    utils::user_utils::{get_possession_suffix, get_target_user, get_user_name},
};
use poise::CreateReply;
use serenity::all::{Color, CreateActionRow, CreateButton, CreateEmbed, Timestamp, User};

#[poise::command(slash_command)]
pub async fn profile_picture(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    let target_user = get_target_user(&ctx, &user, true).await?;
    let avatar = target_user
        .avatar_url()
        .unwrap_or_else(|| target_user.default_avatar_url())
        .replace("?size=1024", "?size=4096");
    let suffix = get_possession_suffix(&target_user);
    let embed = CreateEmbed::new()
        .title(format!(
            "🖼️ {}{} Avatar",
            get_user_name(&target_user),
            suffix
        ))
        .image(&avatar)
        .color(Color::DARK_PURPLE)
        .timestamp(Timestamp::now());
    let button = CreateButton::new_link(avatar.clone())
        .label("📥 Download Avatar")
        .style(serenity::all::ButtonStyle::Primary);
    let components = vec![CreateActionRow::Buttons(vec![button])];

    ctx.send(
        CreateReply::default()
            .reply(true)
            .embed(embed)
            .components(components),
    )
    .await?;

    Ok(())
}
