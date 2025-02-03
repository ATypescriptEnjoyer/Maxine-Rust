mod commands;
mod config;

use std::env;

use serenity::all::{ActivityData, CreateMessage, Guild};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use lazy_static::lazy_static;
lazy_static! {
    // if `test.json` is in the crate root, next to `Cargo.toml`
    static ref CONFIG: config::Config = config::Config::new(env::var("CONFIG_LOCATION").unwrap_or(str::to_string("/data/")));
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "cat" => Some(commands::cat::run(&command.data.options())),
                "8ball" => Some(commands::eightball::run(&command.data.options())),

                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let guild_count = ready.guilds.len();
        println!("{}  is online in {} guild(s)", ready.user.name, guild_count);
        let nick = &*CONFIG.bot.nickname;
        let status = &*CONFIG
            .bot
            .status
            .replace("{guildsCount}", guild_count.to_string().as_str());

        ctx.set_activity(Some(ActivityData::custom(status)));

        for guild in ctx.cache.guilds() {
            let _ = guild.edit_nickname(&ctx.http, Some(nick)).await;
        }

        let _ = Command::set_global_commands(
            &ctx.http,
            vec![commands::ping::register(), commands::cat::register(), commands::eightball::register()],
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

        let _ = guild.edit_nickname(&ctx.http, Some(&*CONFIG.bot.nickname));

        let general_channel = guild.channels.iter().find(|channel| {
            channel.1.is_text_based() && channel.1.name.to_lowercase() == "general"
        });

        let join_message = format!("Hi, i'm {}, thanks for inviting me!", &*CONFIG.bot.nickname);

        if let Some((_, channel)) = general_channel {
            let _ = channel
                .send_message(&ctx.http, CreateMessage::new().content(join_message))
                .await;
            let status = &*CONFIG.bot.status.replace(
                "{guildsCount}",
                ctx.cache.guilds().len().to_string().as_str(),
            );

            ctx.set_activity(Some(ActivityData::custom(status)));
        }
    }
}

#[tokio::main]
async fn main() {
    // Build our client.
    let mut client = Client::builder(CONFIG.bot.token.clone(), GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
