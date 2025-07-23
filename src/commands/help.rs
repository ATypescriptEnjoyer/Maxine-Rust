use poise::{serenity_prelude as serenity, CreateReply};
use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to get help for"] command: Option<String>,
) -> Result<(), Error> {
    if let Some(cmd_name) = command {
        let command_help = get_command_help(&cmd_name);
        let embed = CreateEmbed::new()
            .title(format!("Help: /{}", cmd_name))
            .description(command_help)
            .footer(CreateEmbedFooter::new("Powered by Maxine"));

        ctx.send(CreateReply::default().embed(embed)).await?;
    } else {
        let embed = CreateEmbed::new()
            .title("Maxine Bot Commands")
            .description("Here are all the available commands:")
            .field(
                "🤖 AI & Language Commands",
                "• `/ask` - Ask me anything using AI\n• `/translate` - Translate messages to English\n• `/tldrify` - Create TLDR summaries\n• `/prompt` - Manage your custom AI prompt",
                false,
            )
            .field(
                "🎨 Fun & Utility Commands",
                "• `/avatar` - Show user avatars\n• `/cat` - Get random cat images\n• `/dog` - Get random dog images\n• `/8ball` - Ask the magic 8ball\n• `/urban` - Look up Urban Dictionary definitions",
                false,
            )
            .field(
                "⏰ Time & Media Commands",
                "• `/time` - Check time for any location\n• `/save` - Download and save videos from URLs",
                false,
            )
            .field(
                "🎨 Customization Commands",
                "• `/setcolour` - Set your Discord name color",
                false,
            )
            .field(
                "💡 Usage Tips",
                "• Use `/help <command>` for detailed help on a specific command\n• Most commands work with both slash commands and prefix commands\n• Context menu commands are available for translate and tldrify",
                false,
            )
            .footer(CreateEmbedFooter::new("Powered by Maxine"));

        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}

fn get_command_help(command: &str) -> String {
    match command.to_lowercase().as_str() {
        "ask" => "**Ask me anything!**\n\nUsage: `/ask <your question>`\n\nThis command uses AI to answer your questions. You can also use `/ask <question> <use_default_prompt>` to use the default system prompt instead of your custom one.\n\nExample: `/ask What is the capital of France?`".to_string(),
        
        "translate" => "**Translate messages to English**\n\nUsage: Right-click on a message → Apps → Translate to English\n\nThis command automatically detects the language of a message and translates it to English using AI.\n\nNote: This is a context menu command, not a slash command.".to_string(),
        
        "tldrify" => "**Create TLDR summaries**\n\nUsage: Right-click on a message → Apps → Create TLDR\n\nThis command creates a concise summary of any message using AI.\n\nNote: This is a context menu command, not a slash command.".to_string(),
        
        "prompt" => "**Manage your custom AI prompt**\n\nUsage:\n• `/prompt set <your custom prompt>` - Set your custom system prompt\n• `/prompt get` - View your current custom prompt\n\nThis allows you to customize how the AI responds to your questions.".to_string(),
        
        "avatar" => "**Show user avatars**\n\nUsage: `/avatar [user]`\n\nDisplays the avatar of yourself or another user. If no user is specified, shows your own avatar.\n\nExample: `/avatar @username`".to_string(),
        
        "cat" => "**Get random cat images**\n\nUsage: `/cat`\n\nFetches a random cat image from The Cat API.\n\nExample: `/cat`".to_string(),
        
        "dog" => "**Get random dog images**\n\nUsage: `/dog`\n\nFetches a random dog image from an API.\n\nExample: `/dog`".to_string(),
        
        "8ball" => "**Ask the magic 8ball**\n\nUsage: `/8ball <question>`\n\nGets a random response from the magic 8ball.\n\nExample: `/8ball Will it rain today?`".to_string(),
        
        "urban" => "**Look up Urban Dictionary definitions**\n\nUsage: `/urban <term>`\n\nSearches Urban Dictionary for definitions of terms.\n\nExample: `/urban yeet`".to_string(),
        
        "time" => "**Check time for any location**\n\nUsage: `/time <location>`\n\nGets the current time for any city or location.\n\nExample: `/time New York`".to_string(),
        
        "save" => "**Download and save videos**\n\nUsage: `/save <url> [start_time] [end_time] [format]`\n\nDownloads videos from URLs and optionally clips them. Supports various formats including MP4, GIF, and WebM.\n\nParameters:\n• `url` - The video URL to download\n• `start_time` - Start of clip (HH:MM:SS format)\n• `end_time` - End of clip (HH:MM:SS format)\n• `format` - Output format (mp4, gif, webm)\n\nExample: `/save https://example.com/video.mp4 00:10 00:20 gif`".to_string(),
        
        "setcolour" => "**Set your Discord name color**\n\nUsage: `/setcolour <color>`\n\nChanges your Discord name color. You can use color names or hex codes.\n\nExample: `/setcolour blue` or `/setcolour #FF0000`".to_string(),
        
        _ => "Command not found. Use `/help` to see all available commands.".to_string(),
    }
}
