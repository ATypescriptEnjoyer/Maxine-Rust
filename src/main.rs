#[forbid(unsafe_code)]
mod commands;
mod config;
mod structs;

use ::serenity::all::{ChannelType, CreateChannel, Message, Reaction, VoiceState};
use rig::providers;
use serenity::all::{ActivityData, CreateMessage, Guild};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use poise::serenity_prelude as serenity;
use regex::Regex;

pub const DATA_DIR: &str = if cfg!(debug_assertions) {
    "./data"
} else {
    "/data"
};

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
            ctx.cache.guilds().len()
        );

        let config = &self.config;

        let _ = guild
            .edit_nickname(&ctx.http, Some(&config.bot.nickname))
            .await;

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

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        if new.channel_id.is_some() {
            let guild_id = new.guild_id.unwrap();
            let guild = ctx.cache.guild(guild_id).unwrap().clone();
            let channel_id = new.channel_id.unwrap();
            let channel = guild.channels.get(&channel_id).unwrap().clone();
            let channel_name = &channel.name;

            let user = new.member.unwrap_or_default();

            if channel_name.to_lowercase() == "temp channel creator" {
                let new_channel = guild
                    .create_channel(
                        &ctx.http,
                        CreateChannel::new(format!("{}'s Channel", user.user.display_name()))
                            .kind(ChannelType::Voice)
                            .category(channel.parent_id.unwrap_or_default()),
                    )
                    .await;
                if new_channel.is_ok() {
                    let _ = user
                        .move_to_voice_channel(&ctx.http, new_channel.unwrap().id)
                        .await;
                }
            }
        }

        if old.is_some() {
            let old_state = old.unwrap();
            let guild_id = old_state.guild_id.unwrap();
            let guild = ctx.cache.guild(guild_id).unwrap().clone();
            let channel_id = old_state.channel_id.unwrap();
            let channel = guild.channels.get(&channel_id).unwrap().clone();
            let channel_name = &channel.name;

            if channel_name.to_lowercase().contains("'s channel") {
                let members = channel.members(&ctx.cache).unwrap();
                if members.is_empty() {
                    let _ = channel.delete(&ctx.http).await;
                }
            }
        }
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot || self.config.twitter_embed_url.is_empty() {
            return;
        }

        let re = Regex::new(r#"(?i)\bhttps?://(?:x\.com|twitter\.com)(\S*)"#)
            .expect("Invalid Regex Provided.");
        let mut urls = vec![];

        for (_, [path]) in re.captures_iter(&message.content).map(|c| c.extract()) {
            urls.push(format!("{0}{1}", self.config.twitter_embed_url, path));
        }

        if urls.is_empty() {
            return;
        }

        let _ = message.reply(&ctx.http, urls.join("; ")).await;
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        if add_reaction.emoji.unicode_eq("🗑️")
            && add_reaction.message_author_id.unwrap_or_default() == ctx.cache.current_user().id
        {
            let _ = add_reaction
                .message(&ctx.http)
                .await
                .expect("Can't get message")
                .delete(&ctx.http)
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Maxine");

    let config = config::Config::new(DATA_DIR.to_string());
    let token = &config.bot.token.to_string();
    let intents = GatewayIntents::all();

    let llm_client =
        providers::openai::Client::from_url("ollama", &format!("{}/v1", &config.ollama.host));

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(format!("{}/database.sqlite", DATA_DIR))
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
                commands::help(),
                commands::urban(),
                commands::ask(),
                commands::save(),
                commands::setcolour(),
                commands::time(),
                commands::translate(),
                commands::tldrify(),
                commands::prompt(),
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
