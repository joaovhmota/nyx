use crate::{
    types::{Context, Error},
    utils::{embed_builder_helper::EmbedBuilderHelper, user_utils::get_target_user},
};
use poise::command;
use serenity::{
    all::{CreateActionRow, CreateButton, User},
    model::{application::ButtonStyle, user::PremiumType},
};

/// Gets someone's (or the author's) profile information.
#[command(slash_command)]
pub async fn about(
    ctx: Context<'_>,
    #[description = "Who you want to know more about. If not provided, show info about the author."]
    user: Option<User>,
) -> Result<(), Error> {
    let target_user = get_target_user(&ctx, &user, true).await?;

    let button = CreateButton::new_link(target_user.face())
        .label("📥 Download Avatar")
        .style(ButtonStyle::Primary);

    let mut embed = EmbedBuilderHelper::new(ctx)
        .await?
        .with_title(format!("🪪 About {}", target_user.display_name()))
        .with_thumbnail(target_user.face())
        .with_field("ID", target_user.id.to_string(), true)
        .with_field("Name", &target_user.name, true)
        .with_field("Display Name", target_user.display_name(), true)
        .with_field(
            "User Type",
            if target_user.bot { "Bot" } else { "User" },
            true,
        )
        .with_field(
            "Created at",
            format!("<t:{}:F>", target_user.created_at().unix_timestamp()),
            true,
        )
        .with_field(
            "Discord Nitro Type",
            match target_user.premium_type {
                PremiumType::None => "None",
                PremiumType::NitroClassic => "Classic",
                PremiumType::Nitro => "Nitro",
                PremiumType::NitroBasic => "Nitro Basic",
                PremiumType::Unknown(_) => "Unknown",
                _ => "Undefined",
            },
            true,
        )
        .with_components(vec![CreateActionRow::Buttons(vec![button])]);

    if let Some(banner_url) = target_user.banner_url() {
        embed = embed.with_image(banner_url);
    }

    ctx.send(embed.into()).await?;

    Ok(())
}
