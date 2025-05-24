use poise::{serenity_prelude as serenity, CreateReply};
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use scraper::{Html, Selector};

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Check the time for a location.
#[poise::command(slash_command, prefix_command)]
pub async fn time(
    ctx: Context<'_>,
    #[description = "Location to check the time for"] location: String,
) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://www.bing.com/search?q=time+in+{}", location))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&response);
    
    let text_selector = Selector::parse(".baselClock .b_focusLabel").unwrap();
    let time_selector = Selector::parse("#digit_time").unwrap();

    let text = document.select(&text_selector).next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default();
    let time = document.select(&time_selector).next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default();

    let embed = CreateEmbed::new()
        .title("Time")
        .description(format!("{} is {}", text, time))
        .footer(CreateEmbedFooter::new("Powered by Maxine"));

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
