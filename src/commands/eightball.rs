use serenity::all::CreateCommandOption;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use rand::seq::IndexedRandom;

pub fn run(_options: &[ResolvedOption]) -> String
{

    [
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
	].choose(&mut rand::rng()).unwrap_or(&"Try again later").to_string()

}

pub fn register() -> CreateCommand {
    CreateCommand::new("8ball").description("8ball answers any question!").add_option(CreateCommandOption::new(serenity::all::CommandOptionType::String, "question", "The question to ask the 8ball."))
}