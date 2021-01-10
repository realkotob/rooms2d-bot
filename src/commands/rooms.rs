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

async fn room_command(ctx: &Context, msg: &Message) -> CommandResult{
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
                                        "https://www.mossylogs.com/r/{}-{}",
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
                                    format!("https://www.mossylogs.com/r/{}", channel_name),
                                )
                                .await?;
                        }
                    };
                }
                None => {
                    // println!("It's not a guild channel!");
                    msg.channel_id
                        .say(&ctx.http, format!("https://www.mossylogs.com/r/general"))
                        .await?;
                }
            };
        }
        None => {
            // println!("Channel for message not found");
            msg.channel_id
                .say(&ctx.http, format!("https://www.mossylogs.com/r/general"))
                .await?;
        }
    };

    Ok(())
}
