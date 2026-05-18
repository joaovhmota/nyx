use crate::types::{Context, Error};
use poise::{CreateReply, command};
use rand::RngExt;
use serenity::{
    all::{Color, CreateEmbed, Timestamp},
    builder::{CreateEmbedAuthor, CreateEmbedFooter},
};

#[command(slash_command)]
pub async fn roll_dice(
    ctx: Context<'_>,
    #[description = "Number of sides each dice will have."] mut sides: u8,
    #[description = "Number of dice to roll."] mut count: u8,
    #[description = "Optional modifier for the roll."] modifier: Option<i32>,
) -> Result<(), Error> {
    sides = u8::max(sides, 2);
    count = u8::max(count, 1);

    let (sum, rolls_string) = {
        let mut rng = rand::rng();
        let mut rolls = Vec::new();
        let mut current_sum: i32 = 0;

        for _ in 0..count {
            let roll = rng.random_range(1..=sides) as i32;
            rolls.push(roll);
            current_sum += roll;
        }

        let rolls_str = rolls
            .iter()
            .map(|r: &i32| r.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        (current_sum, rolls_str)
    };

    let modifier_value = modifier.unwrap_or(0);
    let total = sum + modifier_value;

    let mut embed = CreateEmbed::new()
        .author(
            CreateEmbedAuthor::new(ctx.author().display_name().to_string())
                .icon_url(ctx.author().avatar_url().unwrap()),
        )
        .title("🎲 Roll Results")
        .color(Color::DARK_PURPLE)
        .field("Sides", sides.to_string(), true)
        .field("Quantity", count.to_string(), true)
        .field("Rolls", format!("||{}||", rolls_string), false)
        .field("Result", format!("||{}||", sum), true)
        .timestamp(Timestamp::now())
        .footer(CreateEmbedFooter::new(format!(
            "Rolled by {}",
            ctx.author().name
        )));

    if modifier_value != 0 {
        embed = embed.field("Result with Modifier", format!("||{}||", total), true);
        embed = embed.field(
            "Modifier",
            format!(
                "{}{}",
                if modifier_value > 0 { "+" } else { "" },
                modifier_value
            ),
            true,
        );
    }

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
