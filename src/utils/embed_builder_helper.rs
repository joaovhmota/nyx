use poise::CreateReply;
use serenity::{
    builder::{CreateActionRow, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
    model::{Color, Timestamp},
};

use crate::{
    types::{Context, Error},
    utils::user_utils::{get_possession_suffix, get_target_user},
};

pub struct EmbedBuilderHelper {
    internal_embed: CreateEmbed,
    components: Option<Vec<CreateActionRow>>,
}

impl From<EmbedBuilderHelper> for CreateEmbed {
    fn from(val: EmbedBuilderHelper) -> Self {
        val.internal_embed
    }
}

impl From<EmbedBuilderHelper> for CreateReply {
    fn from(val: EmbedBuilderHelper) -> Self {
        CreateReply::default()
            .embed(val.internal_embed)
            .components(val.components.unwrap_or(vec![]))
    }
}

impl EmbedBuilderHelper {
    pub async fn new(ctx: Context<'_>) -> Result<EmbedBuilderHelper, Error> {
        let command_author = get_target_user(&ctx, &None, false).await?;
        let cache = ctx.cache();
        let current_user = cache.current_user();

        let default_embed = CreateEmbed::default()
            .author(
                CreateEmbedAuthor::new(command_author.display_name())
                    .icon_url(command_author.face()),
            )
            .color(command_author.accent_colour.unwrap_or(Color::BLURPLE))
            .footer(
                CreateEmbedFooter::new(format!(
                    "Dont forget to check out {}{} other commands!",
                    current_user.display_name(),
                    get_possession_suffix(&current_user)
                ))
                .icon_url(current_user.face()),
            )
            .timestamp(Timestamp::now());

        Ok(Self {
            internal_embed: default_embed,
            components: None,
        })
    }

    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.internal_embed = self.internal_embed.title(title);
        self
    }

    pub fn with_description<T: Into<String>>(mut self, description: T) -> Self {
        self.internal_embed = self.internal_embed.description(description);
        self
    }

    pub fn with_field<TTitle: Into<String>, TValue: Into<String>>(
        mut self,
        name: TTitle,
        value: TValue,
        inline: bool,
    ) -> Self {
        self.internal_embed = self.internal_embed.field(name, value, inline);
        self
    }

    pub fn with_thumbnail<T: Into<String>>(mut self, thumbnail_url: T) -> Self {
        self.internal_embed = self.internal_embed.thumbnail(thumbnail_url);
        self
    }

    pub fn with_image<T: Into<String>>(mut self, image_url: T) -> Self {
        self.internal_embed = self.internal_embed.image(image_url);
        self
    }

    pub fn with_components(mut self, components: Vec<CreateActionRow>) -> Self {
        self.components = Some(components);
        self
    }
}
