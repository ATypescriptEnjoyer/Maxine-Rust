use poise::CreateReply;
use serenity::all::CreateEmbed;

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Manage your custom system prompt
#[poise::command(slash_command, prefix_command, subcommands("set", "get"))]
pub async fn prompt(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Please use `/prompt set` or `/prompt get`").await?;
    Ok(())
}

/// Set your custom system prompt
#[poise::command(slash_command, prefix_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Your custom system prompt"] prompt: String,
) -> Result<(), Error> {
    let author = ctx.author();
    let user_id = author.id.to_string();

    // Upsert the prompt
    sqlx::query(
        "INSERT INTO UserSystemPrompts (userId, prompt, createdAt, updatedAt) 
         VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) 
         ON CONFLICT(userId) DO UPDATE SET prompt = ?, updatedAt = CURRENT_TIMESTAMP",
    )
    .bind(&user_id)
    .bind(&prompt)
    .bind(&prompt)
    .execute(&ctx.data().database)
    .await?;

    let embed = CreateEmbed::new()
        .title("Prompt Updated")
        .description("Your custom system prompt has been updated successfully.")
        .field("New Prompt", prompt, false)
        .footer(serenity::all::CreateEmbedFooter::new("Powered by Maxine"));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Get your current custom system prompt
#[poise::command(slash_command, prefix_command)]
pub async fn get(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();
    let user_id = author.id.to_string();

    let prompt_result: Result<(String,), sqlx::Error> =
        sqlx::query_as("SELECT prompt FROM UserSystemPrompts WHERE userId = ?")
            .bind(&user_id)
            .fetch_one(&ctx.data().database)
            .await;

    match prompt_result {
        Ok((prompt,)) => {
            let embed = CreateEmbed::new()
                .title("Your Custom Prompt")
                .description("Here is your current custom system prompt:")
                .field("Prompt", prompt, false)
                .footer(serenity::all::CreateEmbedFooter::new("Powered by Maxine"));

            ctx.send(CreateReply::default().embed(embed)).await?;
        }
        Err(_) => {
            ctx.say("You don't have a custom prompt set yet. Use `/prompt set` to set one.")
                .await?;
        }
    }

    Ok(())
}
