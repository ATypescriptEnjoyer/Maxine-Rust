use poise::CreateReply;
use rig::completion::Prompt;
use serenity::all::CreateEmbed;

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Ask me anything!
#[poise::command(slash_command, prefix_command)]
pub async fn ask(
    ctx: Context<'_>,
    #[description = "Your query"] query: String,
    #[description = "Use the default prompt rather than your custom prompt"]
    use_default_prompt: Option<bool>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let author = ctx.author();
    let user_id = author.id.to_string();
    let user_display_name = author.display_name();
    let mut system_prompt = ctx.data().config.ollama.system_prompt.clone();

    if !use_default_prompt.unwrap_or(false) {
        let user_prompt_record: (String,) =
            sqlx::query_as("SELECT prompt FROM UserSystemPrompts WHERE userId = ?")
                .bind(user_id)
                .fetch_one(&ctx.data().database)
                .await?;

        system_prompt = user_prompt_record.0;
    }

    let llm_response = &ctx
        .data()
        .llm_client
        .agent("gemma3:4b")
        .preamble(&system_prompt)
        .append_preamble("Make your response no longer than 1024 characters")
        .append_preamble(&format!("The users name is {}", &user_display_name))
        .build()
        .prompt(&query)
        .await;

    let response_string = match llm_response {
        Ok(response) => response,
        Err(err) => &err.to_string(),
    };

    let cleaned_response = response_string.split("</think>").last().unwrap().trim();

    let embed = CreateEmbed::new()
        .field(
            format!("{user_display_name} asked"),
            query.to_string(),
            false,
        )
        .field("Response", cleaned_response, false)
        .footer(serenity::all::CreateEmbedFooter::new("Powered by Maxine"));

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
