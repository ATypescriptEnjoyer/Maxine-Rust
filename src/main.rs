mod commands;
mod config;

use rig::providers;
use serenity::all::{ActivityData, CreateInteractionResponseFollowup, CreateMessage, Guild};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

enum CommandValue {
    InteractionResponse(Option<CreateInteractionResponse>),
    InteractionResponseFollowup(Option<CreateInteractionResponseFollowup>),
}

struct Handler {
    config: config::Config,
    llm_client: providers::openai::Client,
    database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let command_name = command.data.name.as_str();
            let command_caller = command.user.display_name();
            println!("Received command interaction \"{command_name}\" from {command_caller}");

            let builder: Option<CommandValue> = match command_name {
                "ask" => Some(CommandValue::InteractionResponseFollowup(
                    commands::ask::run(
                        &command,
                        &ctx,
                        &self.config,
                        &self.llm_client,
                        &self.database,
                    )
                    .await,
                )),
                "avatar" => Some(CommandValue::InteractionResponse(commands::avatar::run(
                    &command.user,
                    &command.data.options(),
                ))),
                "cat" => Some(CommandValue::InteractionResponse(
                    commands::cat::run().await,
                )),
                "dog" => Some(CommandValue::InteractionResponse(
                    commands::dog::run().await,
                )),
                "urban" => Some(CommandValue::InteractionResponse(
                    commands::urban::run(&command.data.options()).await,
                )),
                "8ball" => Some(CommandValue::InteractionResponse(commands::eightball::run(
                    &command_caller,
                    &command.data.options(),
                ))),
                _ => Some(CommandValue::InteractionResponse(Some(
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content("Command not implemented"),
                    ),
                ))),
            };

            match builder {
                Some(CommandValue::InteractionResponse(response)) => {
                    let builder_response = response.expect("Something went wrong, and 'response' was 'None'");
                    if let Err(why) = command.create_response(&ctx.http, builder_response).await {
                        println!("Cannot respond to slash command: {why}");
                        let _ = command.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(why.to_string()))).await;
                    }
                }
                Some(CommandValue::InteractionResponseFollowup(response)) => {
                    let builder_response = response.expect("Something went wrong, and the 'response' was 'None'");
                    if let Err(why) = command.create_followup(&ctx.http, builder_response).await {
                        println!("Cannot respond to slash command: {why}");
                        let _ = command.create_followup(&ctx.http, CreateInteractionResponseFollowup::new().content(why.to_string())).await;
                    }
                }
                None => {
                    let err = "Something went wrong, and the command returned 'None'";
                    println!("{err}");
                    let _ = command.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(err))).await;
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let guild_count = ready.guilds.len();
        println!("{}  is online in {} guild(s)", ready.user.name, guild_count);

        let config = &self.config;

        let nick = &config.bot.nickname;
        let status = &config
            .bot
            .status
            .replace("{guildsCount}", guild_count.to_string().as_str());

        ctx.set_activity(Some(ActivityData::custom(status)));

        for guild in ctx.cache.guilds() {
            let _ = guild.edit_nickname(&ctx.http, Some(nick)).await;
        }

        let _ = Command::set_global_commands(
            &ctx.http,
            vec![
                commands::avatar::register(),
                commands::cat::register(),
                commands::dog::register(),
                commands::eightball::register(),
                commands::urban::register(),
                commands::ask::register(),
            ],
        )
        .await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: std::option::Option<bool>) {
        if let Some(false) = is_new {
            return;
        }

        println!(
            "Joined guild: {}. We're now in {} guild(s)!",
            guild.name,
            ctx.cache.guilds().len().to_string()
        );

        let config = &self.config;

        let _ = guild.edit_nickname(&ctx.http, Some(&config.bot.nickname));

        let general_channel = guild.channels.iter().find(|(_, channel)| {
            channel.is_text_based() && channel.name.to_lowercase() == "general"
        });

        let join_message = format!("Hi, i'm {}, thanks for inviting me!", &config.bot.nickname);

        if let Some((_, channel)) = general_channel {
            let _ = channel
                .send_message(&ctx.http, CreateMessage::new().content(join_message))
                .await;
            let status = &config.bot.status.replace(
                "{guildsCount}",
                ctx.cache.guilds().len().to_string().as_str(),
            );

            ctx.set_activity(Some(ActivityData::custom(status)));
        }
    }
}

#[tokio::main]
async fn main() {
    let config = config::Config::new("./data".to_string());
    let llm_client =
        providers::openai::Client::from_url("ollama", &format!("{}/v1", &config.ollama.host));
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("./data/database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    let handler = Handler {
        config: config.clone(),
        database,
        llm_client,
    };

    // Build our client.
    let mut client: Client = Client::builder(&config.bot.token.clone(), GatewayIntents::all())
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
