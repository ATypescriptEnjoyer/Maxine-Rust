#[derive(serde::Deserialize)]
struct UrbanItem {
    word: String,
    definition: String,
}

#[derive(serde::Deserialize)]
struct UrbanResponse {
    list: Vec<UrbanItem>,
}

use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Queries Urban Dictionary for definitions.
#[poise::command(slash_command, prefix_command)]
pub async fn urban(
    ctx: Context<'_>,
    #[description = "The term to look up in Urban Dictionary"] query: String,
) -> Result<(), Error> {
    let url = format!("https://api.urbandictionary.com/v0/define?term={query}");

    let urbans = reqwest::get(url).await?.json::<UrbanResponse>().await.ok();

    if let Some(urbans) = urbans {
        if !urbans.list.is_empty() {
            let entry = &urbans.list[0];

            let embed = CreateEmbed::new()
                .title(format!("Urban Dictionary: {}", &entry.word))
                .field("Definition", &entry.definition, false)
                .footer(CreateEmbedFooter::new("Powered by Maxine"));

            ctx.send(CreateReply::default().embed(embed)).await?;

            return Ok(());
        }
    }

    ctx.send(CreateReply::default().content("No definition could be found."))
        .await?;

    return Ok(());
}
