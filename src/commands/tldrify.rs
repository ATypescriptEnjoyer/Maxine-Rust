use poise::CreateReply;
use rig::completion::Prompt;
use serenity::all::CreateEmbed;
use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command, subcommands("message", "link"))]
pub async fn tldrify(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Please use `/tldrify message` or `/tldrify link`").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn link(ctx: Context<'_>,
    #[description = "The link to summarize"] link: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !link.starts_with("http") {
        ctx.say("Please provide a valid link").await?;
        return Ok(());
    }

    let response = reqwest::get(&link).await?;
    let body = response.text().await?;

    // Try to parse the <article> tag text if it exists, otherwise fallback to the whole body text
    let article_text = {
        let document = scraper::Html::parse_document(&body);
        let article_selector = scraper::Selector::parse("article").unwrap();
        if let Some(article) = document.select(&article_selector).next() {
            article.text().collect::<Vec<_>>().join(" ").trim().to_string()
        } else {
            // Fallback: try to get all visible text from <body>, or just use the whole body as a last resort
            let body_selector = scraper::Selector::parse("body").unwrap();
            if let Some(body_tag) = document.select(&body_selector).next() {
                body_tag.text().collect::<Vec<_>>().join(" ").trim().to_string()
            } else {
                body.clone()
            }
        }
    };

    let result = tldr_query(ctx, article_text).await?;
    create_embed(ctx, &result).await?;
    Ok(())
}   

// Create a TLDR version of a message
#[poise::command(slash_command, context_menu_command = "Create TLDR")]
pub async fn message(
    ctx: Context<'_>,
    #[description = "The message to summarize"] msg: poise::serenity_prelude::Message,
) -> Result<(), Error> {
    ctx.defer().await?;

    let query = msg.content;
    let result = tldr_query(ctx, query).await?;
    create_embed(ctx, &result).await?;
    Ok(())
} 

async fn tldr_query(ctx: Context<'_>, query: String) -> Result<String, Error> {
    let system_prompt = 
    "You are excellent at creating concise summaries of text. Your goal is to create a TLDR (Too Long; Didn't Read) version that captures the main points while being significantly shorter. You must keep your response under 1024 characters.";
  let user_prompt = format!("Create a TLDR version of this text: {}", query);

  let llm_response = &ctx
      .data()
      .llm_client
      .agent("gemma3:4b")
      .append_preamble(system_prompt)
      .build()
      .prompt(&user_prompt)
      .await?;

  let cleaned_response = llm_response.split("</think>").last().unwrap().trim();

  let mut attempt = 1;
  let mut result = cleaned_response.to_string();
  while result.len() > 1024 {
    println!("Result is too long, summarizing again... (Attempt {})", attempt);
    result = Box::pin(tldr_query(ctx, result.clone())).await?;
    attempt += 1;
  }

  Ok(result)
}

async fn create_embed(ctx: Context<'_>, response: &str) -> Result<(), Error> {
  let embed = CreateEmbed::new()
      .title("TLDR Summary")
      .field("Summary", response, false)
      .footer(serenity::all::CreateEmbedFooter::new("Powered by Maxine"));

  ctx.send(CreateReply::default().embed(embed)).await?;

  Ok(())
}