mod commands;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{
        channel::Message,
        event::ResumedEvent,
        gateway::{Activity, Ready},
        interactions::Interaction,
    },
    prelude::*,
    utils::MessageBuilder,
};
use std::{collections::HashSet, env, sync::Arc};

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use commands::{math::*, meta::*, owner::*, rooms::*};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        _ctx.set_activity(Activity::listening(&String::from("!help")))
            .await;

        // ctx.http.create_guild_application_command();
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if (msg.content == "! Room"
            || msg.content == "! room"
            || msg.content == "! Rooms"
            || msg.content == "! room")
        {
            let _ = commands::rooms::room_command(&ctx, &msg).await;
        } else if (msg.content == "!help"
            || msg.content == "! help"
            || msg.content == "! Help"
            || msg.content == "!Help")
        {
            let embed_field_msg = format!(" {}", {
                if msg.guild_id.unwrap_or_default().as_u64() == &(799783945499443231 as u64) {
                    ""
                } else {
                    "\n\nJoin the [Rooms2D discord](https://discord.gg/Egnyj8hbm5) to discover secret features!"
                }
            });

            // let mut user_in_rooms2d = {
            //     if let Ok(guilds) = msg.author.guilds(&http).await {
            //         for (index, guild) in guilds.into_iter().enumerate() {
            //             if (guild.guild_id.unwrap_or_default().as_u64()
            //                 == &(799783945499443231 as u64))
            //             {
            //                 true;
            //             };
            //         }
            //         false
            //     };
            // };
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
        };
    }

    //  async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {}
}

#[group]
#[commands(multiply, ping, quit, room, rooms, private)]
struct General;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD.
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
