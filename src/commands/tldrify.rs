use poise::CreateReply;
use rig::completion::Prompt;
use serenity::all::CreateEmbed;
use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Create a TLDR version of a message
#[poise::command(slash_command, context_menu_command = "Create TLDR")]
pub async fn tldrify(
    ctx: Context<'_>,
    #[description = "The message to summarize"] msg: poise::serenity_prelude::Message,
) -> Result<(), Error> {
    ctx.defer().await?;

    let query = msg.content;

    let system_prompt = 
      "You are excellent at creating concise summaries of text. Your goal is to create a TLDR (Too Long; Didn't Read) version that captures the main points while being significantly shorter.";
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

    let embed = CreateEmbed::new()
        .title("TLDR Summary")
        .field("Summary", cleaned_response, false)
        .footer(serenity::all::CreateEmbedFooter::new("Powered by Maxine"));

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
} 