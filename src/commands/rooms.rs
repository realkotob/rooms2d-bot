use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn rooms(ctx: &Context, msg: &Message) -> CommandResult {
    room_command(ctx, msg).await
}

#[command]
async fn room(ctx: &Context, msg: &Message) -> CommandResult {
    room_command(ctx, msg).await
}

#[command]
async fn private(ctx: &Context, msg: &Message) -> CommandResult {
    private_command(ctx, msg).await
}
async fn private_command(ctx: &Context, msg: &Message) -> CommandResult {
    let embed_field_msg = format!(" {}", {
        if msg.guild_id.unwrap_or_default().as_u64() == &(799783945499443231 as u64) {
            ""
        } else {
            "\n\nJoin the [Rooms2D discord](https://discord.gg/Egnyj8hbm5) to discover secret features!"
        }
    });
    let dm_result = msg
                .author
                .dm(&ctx.http, |m| {
                    m.content("");
                    // m.tts(true);

                    m.embed(|mut e| {
                        e.title("Rooms2D");
                        e.url("https://rooms2d.com");
                        // e.description("Available commands");
                        e.field(
                            "Available commands",
                            format!(
                                "**room** - create a Rooms2D for a channel.\n**private** - create link to a new private room.{}", embed_field_msg),
                            false,
                        );
                        e
                    });
                    m
                })
                .await;

    if let Err(why) = dm_result {
        println!("Error sending help message: {:?}", why);
    } else {
        let _ = msg.react(&ctx, '✅').await;
    };

    Ok(())
}

pub async fn room_command(ctx: &Context, msg: &Message) -> CommandResult {
    let user_name = &msg.author.name;
    let user_name: String = user_name
        .chars()
        .filter_map(|x| match x {
            'A'..='Z' => Some(x),
            'a'..='z' => Some(x),
            '0'..='9' => Some(x),
            ' ' | '-' | ',' => Some('-'),
            '_' => Some('_'),
            _ => None,
        })
        .collect();

    let channel_op = msg.channel(&ctx).await;
    match channel_op {
        Some(channel) => {
            match channel.guild() {
                Some(channel_guild) => {
                    // println!("It's a guild channel named {}!", channel_guild.name);
                    let channel_name = &channel_guild.name;
                    let channel_name: String = channel_name
                        .chars()
                        .filter_map(|x| match x {
                            'A'..='Z' => Some(x),
                            'a'..='z' => Some(x),
                            '0'..='9' => Some(x),
                            ' ' | '-' | ',' => Some('-'),
                            '_' => Some('_'),
                            _ => None,
                        })
                        .collect();

                    let parent_guild_op = channel_guild.guild(&ctx).await;

                    match parent_guild_op {
                        Some(parent_guild) => {
                            // println!("It's a guild channel named {}!", channel_guild.name);
                            let guild_name = &parent_guild.name;

                            let guild_name: String = guild_name
                                .chars()
                                .filter_map(|x| match x {
                                    'A'..='Z' => Some(x),
                                    'a'..='z' => Some(x),
                                    '0'..='9' => Some(x),
                                    ' ' | '-' | ',' => Some('-'),
                                    '_' => Some('_'),
                                    _ => None,
                                })
                                .collect();

                            msg.channel_id
                                .say(
                                    &ctx.http,
                                    format!(
                                        "https://www.rooms2d.com/r/{}-{}",
                                        guild_name, channel_name
                                    ),
                                )
                                .await?;
                        }
                        None => {
                            // println!("Parent guild name not found!");

                            msg.channel_id
                                .say(
                                    &ctx.http,
                                    format!("https://www.rooms2d.com/r/{}", channel_name),
                                )
                                .await?;
                        }
                    };
                }
                None => {
                    // println!("It's not a guild channel!");

                    msg.channel_id
                        .say(
                            &ctx.http,
                            format!("https://www.rooms2d.com/r/{}", user_name),
                        )
                        .await?;
                }
            };
        }
        None => {
            // println!("Channel for message not found");

            msg.channel_id
                .say(
                    &ctx.http,
                    format!("https://www.rooms2d.com/r/{}", user_name),
                )
                .await?;
        }
    };

    Ok(())
}
