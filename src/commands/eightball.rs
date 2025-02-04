use serenity::all::{
    CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
};
use serenity::builder::CreateCommand;

use rand::seq::IndexedRandom;

pub fn run(caller: &str, options: &[ResolvedOption]) -> Option<CreateInteractionResponse> {
	let question = if let Some(ResolvedOption {
        value: ResolvedValue::String(question),
        ..
    }) = options.iter().find(|opt| opt.name == "question")
    {
        question
    } else {
        return None;
    };

    let response = [
        "It is certain",
        "Without a doubt",
        "Definitely",
        "Most likely",
        "Outlook good",
        "Yes!",
        "Try again",
        "Reply hazy",
        "Can't predict",
        "No!",
        "Unlikely",
        "Sources say no",
        "Very doubtful",
    ]
    .choose(&mut rand::rng())
    .unwrap_or(&"Try again later")
    .to_string();

    let embed = CreateEmbed::new()
        .title("Magic 8ball")
        .field(format!("{} asked", caller), question.to_string(), false)
        .field("The 8ball says", &response, false)
        .footer(CreateEmbedFooter::new("Powered by Maxine"));
    let data = CreateInteractionResponseMessage::new().embed(embed);
    Some(CreateInteractionResponse::Message(data))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("8ball")
        .description("8ball answers any question!")
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::String,
            "question",
            "The question to ask the 8ball.",
        ))
}
