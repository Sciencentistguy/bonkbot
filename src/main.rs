use log::*;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use structopt::StructOpt;

struct Handler;

#[tokio::main]
async fn main() {
    // Set default log level to info unless otherwise specified.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "bonkbot=info");
    }
    pretty_env_logger::init();
    let opts = Opt::from_args();
    let token = if opts.token.is_some() {
        opts.token.unwrap()
    } else if opts.token_filename.is_some() {
        std::fs::read_to_string(opts.token_filename.unwrap()).expect("File does not exist")
    } else {
        env::var("DISCORD_TOKEN")
            .expect("Expected either --token, --token-filename, or a token in the environment")
    };

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!bonk") {
            if msg.content == "!bonk" {
                msg.reply(&ctx, "Usage: `!bonk <text>`")
                    .await
                    .expect("Failed to send message");
                return;
            }
            let bonk_text = &msg.content[6..msg.content.len()];
            info!("Sending bonk message with content '{}'", bonk_text);
            msg.channel_id
                .say(
                    &ctx,
                    format!("Bonk! {} go to horny jail. <:BonkaS:811597040302948382>", bonk_text),
                )
                .await
                .expect("Failed to send message");
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "bonkbot",
    about = "A small silly bot to \"bonk\" people in discord"
)]
struct Opt {
    /// Provide the token
    #[structopt(short, long)]
    token: Option<String>,
    /// Provide the name of a file containing the token
    #[structopt(short = "f", long)]
    token_filename: Option<String>,
}
