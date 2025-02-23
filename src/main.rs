#[forbid(unsafe_code)]
mod commands;
mod config;
mod structs;

use rig::providers;
use serenity::all::{ActivityData, CreateMessage, Guild};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use poise::serenity_prelude as serenity;

#[async_trait]
impl EventHandler for structs::Handler {
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
    println!("Starting Maxine");

    let config = config::Config::new("./data".to_string());
    let token = &config.bot.token.to_string();
    let intents = GatewayIntents::all();

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

    let handler = structs::Handler {
        config: config.clone(),
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::avatar(),
                commands::cat(),
                commands::dog(),
                commands::eightball(),
                commands::urban(),
                commands::ask(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(structs::Data {
                    config: config.clone(),
                    database,
                    llm_client,
                })
            })
        })
        .build();

    println!("Framework Built");

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(handler)
        .await
        .unwrap();

    client.start().await.unwrap();
    print!("Maxine is Online");
}
