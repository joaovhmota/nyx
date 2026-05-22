use crate::{
    types::{Context, Error},
    utils::{
        embed_builder_helper::EmbedBuilderHelper,
        user_utils::{get_possession_suffix, get_target_user},
    },
};
use serenity::{
    all::{CreateActionRow, CreateButton, User},
    model::application::ButtonStyle,
};

/// Gets someone's (or the author's) profile picture.
#[poise::command(slash_command)]
pub async fn profile_picture(
    ctx: Context<'_>,
    #[description = "User to get profile picture. If not provided, show info about the author."]
    user: Option<User>,
) -> Result<(), Error> {
    let target_user = get_target_user(&ctx, &user, true).await?;
    let suffix = get_possession_suffix(&target_user);
    let button = CreateButton::new_link(target_user.face().clone())
        .label("📥 Download Avatar")
        .style(ButtonStyle::Primary);
    let embed = EmbedBuilderHelper::new(ctx)
        .await?
        .with_title(format!(
            "🖼️ {}{} profile picture",
            target_user.display_name(),
            suffix
        ))
        .with_image(target_user.face().clone())
        .with_components(vec![CreateActionRow::Buttons(vec![button])]);

    ctx.send(embed.into()).await?;

    Ok(())
}
