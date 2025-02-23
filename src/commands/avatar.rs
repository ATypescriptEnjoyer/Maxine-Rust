use poise::{serenity_prelude as serenity, CreateReply};

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's Avatar.
#[poise::command(slash_command, prefix_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "User to show Avatar for"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let user_name = user.display_name();
    let avatar_url = user.avatar_url().expect("Can't retrieve user Avatar.");

    let embed = serenity::CreateEmbed::new()
        .title(format!("Avatar for {user_name}"))
        .image(avatar_url)
        .footer(serenity::CreateEmbedFooter::new("Powered by Maxine"));

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
