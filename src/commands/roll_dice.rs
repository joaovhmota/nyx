use crate::{
    types::{Context, Error},
    utils::embed_builder_helper::EmbedBuilderHelper,
};
use poise::command;
use rand::RngExt;
use serenity::all::Color;

/// Roll's a dice, with custom sizes and amounts (optionally with a modifier).
#[command(slash_command)]
pub async fn roll_dice(
    ctx: Context<'_>,
    #[description = "Number of sides each dice will have."] mut sides: u8,
    #[description = "Number of dice to roll."] mut count: u8,
    #[description = "Hides the final results"] hide_results: Option<bool>,
    #[description = "Optional modifier for the roll."] modifier: Option<i32>,
) -> Result<(), Error> {
    sides = u8::max(sides, 2);
    count = u8::max(count, 1);

    let (raw_sum, sum_string) = {
        let mut rng = rand::rng();
        let mut rolls = Vec::new();
        let mut current_sum: i32 = 0;

        for _ in 0..count {
            let roll = rng.random_range(1..=sides) as i32;
            rolls.push(roll);
            current_sum += roll;
        }

        let sum_string = rolls
            .iter()
            .map(|&roll_value| roll_value.to_string())
            .collect::<Vec<String>>()
            .join(" + ");

        (current_sum, sum_string)
    };

    let modifier_value = modifier.unwrap_or(0);
    let final_result = raw_sum + modifier_value;

    let mut new_embed = EmbedBuilderHelper::new(ctx)
        .with_title("🎲 Roll Results")
        .with_color(Color::DARK_PURPLE)
        .with_field("Sides", sides.to_string(), true)
        .with_field("Quantity", count.to_string(), true)
        .with_field("Rolls", sum_string, false)
        .with_field(
            "Total",
            {
                if hide_results.unwrap_or(false) {
                    format!("||{raw_sum}||")
                } else {
                    raw_sum.to_string()
                }
            },
            true,
        );

    if modifier_value != 0 {
        new_embed = new_embed
            .with_field(
                "Modifier",
                format!(
                    "{}{}",
                    if modifier_value > 0 { "+" } else { "-" },
                    modifier_value
                ),
                true,
            )
            .with_field(
                "Total + Modifier",
                {
                    {
                        if hide_results.unwrap_or(false) {
                            format!("||{final_result}||")
                        } else {
                            final_result.to_string()
                        }
                    }
                },
                true,
            );
    }

    ctx.send(new_embed.into()).await?;

    Ok(())
}
