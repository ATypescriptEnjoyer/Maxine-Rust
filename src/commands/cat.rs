use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Sorry, no cat yet.".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cat").description("Cat")
}