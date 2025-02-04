use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
};

#[derive(serde::Deserialize)]
struct UrbanItem {
    word: String,
    definition: String,
}

#[derive(serde::Deserialize)]
struct UrbanResponse {
    list: Vec<UrbanItem>,
}

pub async fn run(options: &[ResolvedOption<'_>]) -> Option<CreateInteractionResponse> {
    // Extract query parameter using pattern matching
    let query = if let Some(ResolvedOption {
        value: ResolvedValue::String(query),
        ..
    }) = options.iter().find(|opt| opt.name == "query")
    {
        query
    } else {
        return None;
    };

    // Fetch and parse urban dictionary response
    let urban_response: UrbanResponse = reqwest::get(format!(
        "https://api.urbandictionary.com/v0/define?term={query}"
    ))
    .await
    .ok()?
    .json::<UrbanResponse>()
    .await
    .ok()?;

    // Get first result or return None
    let first_entry = urban_response.list.into_iter().next();

    let mut embed = CreateEmbed::new().footer(CreateEmbedFooter::new("Powered by Maxine"));

    match first_entry {
        Some(entry) => {
            embed = embed
                .title(format!("Urban Dictionary: {}", entry.word))
                .field("Definition", &entry.definition, false);
        }
        None => {
            embed = embed
                .title(format!("Urban Dictionary: {}", query))
                .field("Definition", "No definition could be found.", false)
        }
    }

    let data = CreateInteractionResponseMessage::new().embed(embed);
    Some(CreateInteractionResponse::Message(data))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("urban")
        .description("Queries Urban Dictionary for definitions")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "query",
                "The term to look up in Urban Dictionary",
            )
            .required(true),
        )
}
