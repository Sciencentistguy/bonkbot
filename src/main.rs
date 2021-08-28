use log::*;
use std::path::PathBuf;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOptionValue;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::interactions::Interaction;
use serenity::model::interactions::InteractionResponseType;
use serenity::prelude::*;

use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use structopt::StructOpt;

static BONK_EMOJIS: Lazy<Vec<&'static str>> =
    Lazy::new(|| include_str!("bonk_emojis.txt").lines().collect());

const DISCORD_MESSAGE_MAX_LENGTH: usize = 2000;

struct Handler;

#[tokio::main]
async fn main() {
    // Set default log level to info unless otherwise specified.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "bonkbot=info");
    }

    pretty_env_logger::init();

    let opts = Opt::from_args();

    let token = std::fs::read_to_string(opts.token_filename).expect("File does not exist");

    let appid = std::fs::read_to_string(opts.application_id_filename)
        .expect("File does not exist")
        .trim()
        .parse::<u64>()
        .unwrap();

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .application_id(appid)
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
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "bonk" => {
                    let user = match command
                        .data
                        .options
                        .get(0)
                        .and_then(|o| o.resolved.as_ref())
                    {
                        Some(ApplicationCommandInteractionDataOptionValue::User(user, _)) => user,
                        _ => panic!("Expected a user argument"),
                    };

                    let bonk_emoji = {
                        let mut rng = rand::rngs::OsRng::default();
                        *BONK_EMOJIS.choose(&mut rng).unwrap()
                    };

                    format!("Bonk! {} go to horny jail. {}", user.mention(), bonk_emoji)
                }
                _ => "Error: invalid command name".to_owned(),
            };

            command
                .create_interaction_response(&ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.content(content))
                })
                .await
                .expect("Failed to respond to slash command");
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!bonk") {
            if msg.content == "!bonk" {
                msg.reply(&ctx, "Usage: `!bonk <text>`")
                    .await
                    .expect("Failed to send message");
                return;
            }
            if msg.content.len() > DISCORD_MESSAGE_MAX_LENGTH {
                error!("Message too long");
                msg.reply(
                    &ctx,
                    "Error: bonk message would be too long. Stop trying to break my bot 😠",
                )
                .await
                .expect("failed to reply to message");
                return;
            }
            let bonk_text = &msg.content[6..msg.content.len()];
            let bonk_emoji = {
                let mut rng = rand::rngs::OsRng::default();
                *BONK_EMOJIS.choose(&mut rng).unwrap()
            };
            info!(
                "Sending bonk message with content '{}' and emoji '{}'",
                bonk_text, bonk_emoji
            );
            msg.channel_id
                .say(
                    &ctx,
                    format!("Bonk! {} go to horny jail. {}", bonk_text, bonk_emoji),
                )
                .await
                .expect("Failed to send message");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx, |builder| {
            builder.create_application_command(|command_builder| {
                command_builder
                    .name("bonk")
                    .description("Send someone to horny jail")
                    .create_option(|option_builder| {
                        option_builder
                            .name("user")
                            .description("The user to bonk")
                            .kind(ApplicationCommandOptionType::User)
                            .required(true)
                    })
            })
        })
        .await
        .expect("Command builder failed");

        info!(
            "Registered the following slash commands: {:?}",
            commands
                .iter()
                .map(|cmd| cmd.name.as_str())
                .collect::<Vec<_>>()
        );
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "bonkbot",
    about = r#"A small silly bot to "bonk" people in discord"#
)]
struct Opt {
    /// File containing the bot token
    #[structopt()]
    token_filename: PathBuf,
    /// File containing the application id
    #[structopt()]
    application_id_filename: PathBuf,
}
