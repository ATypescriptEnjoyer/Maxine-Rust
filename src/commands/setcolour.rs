use ::serenity::all::{Colour, EditRole};
use colors_transform::{Color, Rgb};
use poise::{serenity_prelude as serenity, CreateReply};
use rig::completion::Prompt;
use serenity::all::{CreateEmbed, CreateEmbedFooter, RoleId};

use crate::structs::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Sets your discord name colour :)
#[poise::command(slash_command, prefix_command)]
pub async fn setcolour(
    ctx: Context<'_>,
    #[description = "Discord recognisable colour code"] colour_code: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let guild = ctx
        .guild()
        .ok_or("Command must be used in a guild")?
        .clone();
    let guild_user = guild
        .members
        .get(&ctx.author().id)
        .ok_or("Could not find user")?
        .clone();

    // Convert color name to hex and then to RGB
    let llm_response = &ctx
        .data()
        .llm_client
        .agent("gemma3:4b")
        .append_preamble("You are a helpful assistant that converts color names to hex values. Respond with ONLY the hex value, nothing else.")
        .build()
        .prompt(&format!("Convert this color to a hex value: {}", colour_code))
        .await?;

    let cleaned_response = llm_response.split("</think>").last().unwrap().trim();

    let rgb = Rgb::from_hex_str(cleaned_response).unwrap();
    let colour = Colour::from_rgb(
        rgb.get_red() as u8,
        rgb.get_green() as u8,
        rgb.get_blue() as u8,
    );

    // Check role hierarchy
    let bot_member = guild
        .members
        .get(&ctx.cache().current_user().id)
        .ok_or("Could not find bot")?;
    let bot_topmost_role = bot_member
        .roles
        .iter()
        .filter_map(|role_id| guild.roles.get(role_id))
        .map(|role| role.position)
        .max()
        .unwrap_or(0);

    let users_topmost_role = guild_user
        .roles
        .iter()
        .filter_map(|role_id| guild.roles.get(role_id))
        .filter(|role| role.colour.0 > 0)
        .map(|role| role.position)
        .max()
        .unwrap_or(0);

    if users_topmost_role >= bot_topmost_role {
        return send_embed(
            ctx,
            "Colour Update Failed",
            "You have a role higher than me, so I can't assign your colour role.",
            colour,
        )
        .await;
    }

    // Handle role management
    let colour_key = format!("CLR-{}", colour_code);

    // Remove existing colour roles
    let users_colour_roles: Vec<RoleId> = guild_user
        .roles
        .iter()
        .filter_map(|role_id| {
            guild
                .roles
                .get(role_id)
                .filter(|role| role.name.starts_with("CLR-"))
                .map(|_| *role_id)
        })
        .collect();

    if !users_colour_roles.is_empty() {
        guild_user
            .remove_roles(&ctx.http(), &users_colour_roles)
            .await?;
    }

    // Create or get the colour role
    let new_colour_role =
        if let Some(role) = guild.roles.values().find(|role| role.name == colour_key) {
            role.clone()
        } else {
            let role = EditRole::new()
                .name(&colour_key)
                .colour(colour)
                .position(users_topmost_role + 1);
            guild.create_role(&ctx.http(), role).await?
        };

    // Add the new role to the user
    guild_user.add_role(&ctx.http(), new_colour_role.id).await?;

    // Clean up empty colour roles
    for role in guild.roles.values() {
        if role.name.starts_with("CLR-")
            && !guild
                .members
                .values()
                .any(|member| member.roles.contains(&role.id))
        {
            guild.delete_role(&ctx.http(), role.id).await?;
        }
    }

    send_embed(
        ctx,
        "Colour Updated",
        "Your name colour has been updated!",
        colour,
    )
    .await
}

async fn send_embed(
    ctx: Context<'_>,
    title: &str,
    description: &str,
    colour: Colour,
) -> Result<(), Error> {
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title(title)
                .description(description)
                .footer(CreateEmbedFooter::new("Powered by Maxine"))
                .colour(colour),
        ),
    )
    .await?;
    Ok(())
}
