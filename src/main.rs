mod commands;
mod mongo;
mod scanners;
mod messages;
mod utils;

use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use moka::future::Cache;
use mongo::mongo_repo::MongoRepo;
use mongo::servers_settings::ServerSettings;
use mongodb::bson::oid::ObjectId;
use serenity::all::{ActivityData, CreateEmbed, CreateEmbedFooter, CreateMessage, EmbedFooter, Guild, Message, ShardManager};
use serenity::model::guild;
use utils::send_log;
use crate::mongo::scanner::ScannerType;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler {
    db: Arc<MongoRepo>
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = &interaction {
            println!("Received command interaction: {command:#?}");
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "id" => Some(commands::id::run(&command.data.options())),
                "addregexrule" => Some(commands::add_regex_rule::run(&ctx, &interaction.clone(), &command.data.options(), Arc::clone(&self.db)).await),
                "setup" => Some(commands::setup::run(&ctx, &interaction.clone(), &command.data.options(), Arc::clone(&self.db)).await),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content).ephemeral(true);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        println!("{} has been added to the guild {}", ctx.cache.current_user().name, guild.name);
        let embed = CreateEmbed::default()
            .title("Hello! Im PresenceBot!")
            .description("I am a bot that can help you moderate your server! \n To start using the bot, please add a location for a bot to log to by using the /setup command!")
            .footer(CreateEmbedFooter::new("This message was auto-generated when I was added to the server.".to_string()));
        if !guild.system_channel_id.is_none() {
            guild.system_channel_id.unwrap().send_message(&ctx.http, CreateMessage::new().add_embed(embed)).await.unwrap();
        }
        else {
            guild.channels.iter().next().unwrap().1.id.send_message(&ctx.http, CreateMessage::new().add_embed(embed)).await.unwrap();
        }
        let _ = self.db.create_server_settings(ServerSettings {_id: ObjectId::new(), discord_id: guild.id.to_string(), log_channel: "".to_string()});
        
    }

    async fn message(&self, ctx: Context, msg: Message) {
        match msg.guild_id {Some(_) => {} None => {return;}}
        let scanners = &self.db.get_scanners_by_disid(msg.guild_id.unwrap().to_string()).await.unwrap();
        for scanner in scanners {
            match scanner.scanner_backend {
                ScannerType::Pattern(ref pattern) => {
                    if pattern.is_match(&msg.content) {
                        let _ = msg.author.dm(&ctx.http, messages::block::blockedmessage(&msg.content, &scanner._id.to_hex())).await;
                        msg.delete(&ctx.http).await.unwrap();
                        send_log::send_log(msg.clone(), msg.author.clone(), scanner._id.to_hex(), ctx.clone(), Arc::clone(&self.db)).await;
                    }
                }
                ScannerType::Word(ref word) => {
                    if word.is_match(&msg.content) {
                        let _ = msg.author.dm(&ctx.http, messages::block::blockedmessage(&msg.content, &scanner._id.to_hex())).await;
                        msg.delete(&ctx.http).await.unwrap();
                        send_log::send_log(msg.clone(), msg.author.clone(), scanner._id.to_hex(), ctx.clone(), Arc::clone(&self.db)).await;
                    }
                }
            }
        }

    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        dotenv().ok();
        /*let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
                commands::id::register(),
                commands::welcome::register(),
                commands::attachmentinput::register(),
            ])
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");
        */
        let _guild_command =
            Command::create_global_command(&ctx.http, commands::setup::register())
                .await;
        let global_command = Command::create_global_command(&ctx.http, commands::add_regex_rule::register())
                .await;
        println!("I created the following global slash command: {global_command:#?}");

        // println!("I created the following global slash command: {guild_command:#?}");
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");
    let cache: Arc<Cache<String, String>> = Arc::new(Cache::new(1000));
    let database = Arc::new(MongoRepo::init(cache).await);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler {db: database})
        .await
        .expect("Error creating client");
    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
