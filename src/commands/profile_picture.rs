use crate::{
    types::{Context, Error},
    utils::{
        embed_builder_helper::EmbedBuilderHelper,
        user_utils::{get_possession_suffix, get_target_user, get_user_name},
    },
};
use serenity::all::{Color, CreateActionRow, CreateButton, User};

/// Gets someone's (or the author's) profile picture.
#[poise::command(slash_command)]
pub async fn profile_picture(
    ctx: Context<'_>,
    #[description = "User to get profile picture. If not provided, show info about the author."]
    user: Option<User>,
) -> Result<(), Error> {
    let target_user = get_target_user(&ctx, &user, true).await?;
    let avatar = target_user
        .avatar_url()
        .unwrap_or_else(|| target_user.default_avatar_url())
        .replace("?size=1024", "?size=4096");
    let suffix = get_possession_suffix(&target_user);
    let button = CreateButton::new_link(avatar.clone())
        .label("📥 Download Avatar")
        .style(serenity::all::ButtonStyle::Primary);
    let embed = EmbedBuilderHelper::new(ctx)
        .with_title(format!(
            "🖼️ {}{} Avatar",
            get_user_name(&target_user),
            suffix
        ))
        .with_description(format!(
            "Below is {}{} profile picture, use the button below to download it.",
            get_user_name(&target_user),
            suffix
        ))
        .with_image(avatar.clone())
        .with_color(Color::DARK_PURPLE)
        .with_components(vec![CreateActionRow::Buttons(vec![button])]);

    ctx.send(embed.into()).await?;

    Ok(())
}
