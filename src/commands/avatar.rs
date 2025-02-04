use serenity::all::{
    CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, ResolvedOption, ResolvedValue, User,
};
use serenity::builder::CreateCommand;

pub fn run(calling_user: &User, options: &[ResolvedOption]) -> Option<CreateInteractionResponse> {
    let user = if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _),
        ..
    }) = options.iter().find(|opt| opt.name == "user")
    {
        user
    } else {
        &calling_user
    };

    let user_name = user.display_name();
    let avatar_url = user.avatar_url()?;

    let embed = CreateEmbed::new()
        .title(format!("Avatar for {user_name}"))
        .image(avatar_url)
        .footer(CreateEmbedFooter::new("Powered by Maxine"));
    let data = CreateInteractionResponseMessage::new().embed(embed);
    Some(CreateInteractionResponse::Message(data))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("avatar")
        .description("Gets the avatar for a specific user")
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::User,
            "user",
            "User to retrieve avatar for",
        ))
}
