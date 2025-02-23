use poise::{serenity_prelude as serenity, CreateReply};
use serenity::all::{
    CreateEmbed, CreateEmbedFooter,
};

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(serde::Deserialize)]
struct Cat {
    url: String,
}

/// Get a random cat image.
#[poise::command(slash_command, prefix_command)]
pub async fn cat(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let cat = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await?
        .json::<Vec<Cat>>()
        .await?;

    if cat.len() > 0 {
        let embed = CreateEmbed::new()
            .title("Here's your cat!")
            .image(&cat[0].url)
            .footer(CreateEmbedFooter::new("Powered by Maxine"));

        ctx.send(CreateReply::default().embed(embed)).await?;

        return Ok(());
    }

    ctx.reply("Unable to fetch cat photo :(").await?;
    Ok(())
}
