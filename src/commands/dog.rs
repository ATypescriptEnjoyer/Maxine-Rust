use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::CreateCommand;

#[derive(serde::Deserialize)]
struct Dog {
    message: String,
}

pub async fn run() -> Option<CreateInteractionResponse> {
    let dog = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await.ok()?
        .json::<Dog>()
        .await.ok()?;

    let embed = CreateEmbed::new().title("Here's your dog!").image(dog.message).footer(CreateEmbedFooter::new("Powered by Maxine"));
    let data = CreateInteractionResponseMessage::new().embed(embed);
    Some(CreateInteractionResponse::Message(data))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("dog").description("Get a random dog image")
}