use rand::seq::IndexedRandom;

use poise::{serenity_prelude as serenity, CreateReply};
use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// 8ball answers any question!.
#[poise::command(slash_command, prefix_command, rename = "8ball")]
pub async fn eightball(
    ctx: Context<'_>,
    #[description = "The question to ask the 8ball."] question: String,
) -> Result<(), Error> {
    let response = [
        "It is certain",
        "Without a doubt",
        "Definitely",
        "Most likely",
        "Outlook good",
        "Yes!",
        "Try again",
        "Reply hazy",
        "Can't predict",
        "No!",
        "Unlikely",
        "Sources say no",
        "Very doubtful",
    ]
    .choose(&mut rand::rng())
    .unwrap_or(&"Try again later")
    .to_string();

    let embed = CreateEmbed::new()
        .title("Magic 8ball")
        .field(
            format!("{} asked", &ctx.author().display_name()),
            question.to_string(),
            false,
        )
        .field("The 8ball says", &response, false)
        .footer(CreateEmbedFooter::new("Powered by Maxine"));

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
