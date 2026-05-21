use crate::{
    types::{Context, Error},
    utils::{
        embed_builder_helper::EmbedBuilderHelper,
        user_utils::{get_target_user, get_user_name},
    },
};
use poise::command;
use serenity::all::{Color, CreateActionRow, CreateButton, User};

/// Get's someone's (or the author's) profile information.
#[command(slash_command)]
pub async fn about(
    ctx: Context<'_>,
    #[description = "Who you want to know more about. If not provided, show info about the author."]
    user: Option<User>,
) -> Result<(), Error> {
    let target_user = get_target_user(&ctx, &user, true).await?;
    let avatar = target_user
        .avatar_url()
        .unwrap_or(target_user.default_avatar_url())
        .replace("?size=1024", "?size=4096");
    let user_id = target_user.id;
    let created_at = target_user.created_at();
    let is_bot = target_user.bot;

    let button = CreateButton::new_link(avatar.clone())
        .label("📥 Download Avatar")
        .style(serenity::all::ButtonStyle::Primary);

    let embed = EmbedBuilderHelper::new(ctx)
        .with_title(format!("🪪 About {}", get_user_name(&target_user)))
        .with_description(format!(
            "Know more about {} with this summary of their profile information.",
            get_user_name(&target_user)
        ))
        .with_thumbnail(&avatar)
        .with_color(Color::DARK_PURPLE)
        .with_field("ID", user_id.to_string(), true)
        .with_field("Name", get_user_name(&target_user), true)
        .with_field("Type", if is_bot { "Bot" } else { "User" }, true)
        .with_field(
            "Created at",
            format!("<t:{}:F>", created_at.unix_timestamp()),
            false,
        )
        .with_components(vec![CreateActionRow::Buttons(vec![button])]);

    ctx.send(embed.into()).await?;

    Ok(())
}
