use rig::completion::Prompt;
use rig::providers;
use serenity::all::{
    CommandInteraction, Context, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponseFollowup, ResolvedOption, ResolvedValue
};
use serenity::builder::CreateCommand;

use crate::config;

pub async fn run(
    command: &CommandInteraction,
    ctx: &Context,
    config: &config::Config,
    client: &providers::openai::Client,
    database: &sqlx::SqlitePool,
) -> Option<CreateInteractionResponseFollowup> {
    command.defer(&ctx.http).await.ok()?;
    let options: &[ResolvedOption<'_>] = &command.data.options();

    let query = if let Some(ResolvedOption {
        value: ResolvedValue::String(query),
        ..
    }) = options.iter().find(|opt| opt.name == "query")
    {
        query
    } else {
        return None;
    };

    let user_id = command.user.id.get().to_string();
    let user_display_name = command.user.display_name();
    let user_prompt_record: Option<(String,)> =
        sqlx::query_as("SELECT prompt FROM UserSystemPrompts WHERE userId = ?")
            .bind(user_id)
            .fetch_one(database)
            .await
            .ok();

    let system_prompt = match user_prompt_record {
        Some(row) => row.0,
        None => config.ollama.system_prompt.clone(),
    };

    let llm_response = client
        .agent(&config.ollama.models.instruct)
        .preamble(&system_prompt)
        .append_preamble("Make your response no longer than 1024 characters")
        .append_preamble(&format!(
            "The users name is {}",
            &user_display_name
        ))
        .build()
        .prompt(query)
        .await;

    if llm_response.is_err() {
        return Some(CreateInteractionResponseFollowup::new().content(llm_response.err().unwrap().to_string()))
    }

    let response_string = match llm_response {
        Ok(response) => response,
        Err(_) => "Maxine failed to respond, please try again.".to_string(),
    };

    let embed = CreateEmbed::new()
        .field(format!("{user_display_name} asked"), query.to_string(), false)
        .field("Response", response_string, false)
        .footer(CreateEmbedFooter::new("Powered by Maxine"));

    Some(CreateInteractionResponseFollowup::new().embed(embed))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ask")
        .description("Ask a question!")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::String,
                "query",
                "Your query",
            )
            .required(true),
        )
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::Boolean,
            "search_web",
            "Search web to help assist with accurate results",
        ))
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::Boolean,
            "use_default_prompt",
            "Search web to help assist with accurate results",
        ))
}
