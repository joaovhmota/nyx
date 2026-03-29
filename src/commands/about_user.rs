use crate::{
    types::{Context, Error},
    utils::user_utils::{get_target_user, get_user_name},
};
use poise::{CreateReply, command};
use serenity::all::{Color, CreateActionRow, CreateButton, CreateEmbed, Timestamp, User};

#[command(category = "admin", slash_command, rename = "about_user")]
pub async fn about_user(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    let target_user = get_target_user(&ctx, &user, true).await?;
    let avatar = target_user
        .avatar_url()
        .unwrap_or(target_user.default_avatar_url())
        .replace("?size=1024", "?size=4096");
    let user_id = target_user.id;
    let created_at = target_user.created_at();
    let is_bot = target_user.bot;

    let embed = CreateEmbed::new()
        .title(format!("・About {}", target_user.name))
        .thumbnail(&avatar)
        .color(Color::DARK_PURPLE)
        .field("👤 Name", get_user_name(&target_user), true)
        .field("🪪 ID", user_id.to_string(), true)
        .field("⚙️ Is APP?", if is_bot { "✅" } else { "❌" }, true)
        .field(
            "🗓️ Account Creation Date",
            format!("<t:{}:F>", created_at.unix_timestamp()),
            false,
        )
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
