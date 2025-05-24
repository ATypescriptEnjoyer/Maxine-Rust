use poise::CreateReply;
use rig::completion::Prompt;
use serenity::all::CreateEmbed;
use serde_json;
use serde::{Deserialize, Serialize};
use crate::structs::Data;

#[derive(Debug, Serialize, Deserialize)]
struct TranslationResponse {
  input_language: String,
    translation: String,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Translate a message to English
#[poise::command(slash_command, context_menu_command = "Translate to English")]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "The message to translate"] msg: poise::serenity_prelude::Message,
) -> Result<(), Error> {
    ctx.defer().await?;

    let query = msg.content;

    let system_prompt = 
      "You are excellent at detecting languages and translating text to English. The origin language of the text provided for you to translate will never be English. 
      You MUST Respond EXACTLY in the following JSON format, do not forget the curly braces:

      {
        \"input_language\": \"detected input language\",
        \"translation\": \"the english translation\"
      }
      ";
    let user_prompt = format!("Detect what language and translate this into English: {}", query);

    let llm_response = &ctx
        .data()
        .llm_client
        .agent(&ctx.data().config.ollama.models.tools)
        .append_preamble(system_prompt)
        .build()
        .prompt(&user_prompt)
        .await?;

    let translation: TranslationResponse = serde_json::from_str(llm_response)?;

    let embed = CreateEmbed::new()
        .field(
            format!("LLM Translation from {} to English", translation.input_language),
            translation.translation,
            false,
        )
        .footer(serenity::all::CreateEmbedFooter::new("Powered by Maxine"));

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
