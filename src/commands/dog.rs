use poise::{serenity_prelude as serenity, CreateReply};
use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(serde::Deserialize)]
struct Dog {
    message: String,
}

/// Get a random dog image.
#[poise::command(slash_command, prefix_command)]
pub async fn dog(ctx: Context<'_>) -> Result<(), Error> {
    let dog = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json::<Dog>()
        .await
        .ok();

    if let Some(dog) = dog {
        let embed = CreateEmbed::new()
            .title("Here's your dog!")
            .image(&dog.message)
            .footer(CreateEmbedFooter::new("Powered by Maxine"));

        ctx.send(CreateReply::default().embed(embed)).await?;

        return Ok(());
    }

    ctx.reply("Unable to fetch dog photo :(").await?;
    Ok(())
}
