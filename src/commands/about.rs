use crate::{
    types::{Context, Error},
    utils::user_utils::{get_target_user, get_user_name},
};
use poise::{CreateReply, command};
use serenity::{
    all::{Color, CreateActionRow, CreateButton, CreateEmbed, Timestamp, User},
    builder::CreateEmbedAuthor,
};

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

    let embed = CreateEmbed::new()
        .author(
            CreateEmbedAuthor::new(ctx.author().display_name().to_string())
                .icon_url(ctx.author().avatar_url().unwrap_or_default()),
        )
        .title(format!("🪪 About {}", target_user.name))
        .description(format!(
            "Know more about {} with this summary of their profile information.",
            get_user_name(&target_user)
        ))
        .thumbnail(&avatar)
        .color(Color::DARK_PURPLE)
        .field("ID", user_id.to_string(), true)
        .field("Name", get_user_name(&target_user), true)
        .field("Type", if is_bot { "Bot" } else { "User" }, true)
        .field(
            "Created at",
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
