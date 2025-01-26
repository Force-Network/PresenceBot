mod commands;
mod mongo;
mod scanners;
mod messages;

use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use moka::future::Cache;
use mongo::mongo_repo::MongoRepo;
use serenity::all::{ActivityData, CreateEmbed, CreateMessage, Message, ShardManager};
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
                "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
                "addregexrule" => Some(commands::add_regex_rule::run(&ctx, &interaction.clone(), &command.data.options(), Arc::clone(&self.db)).await),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        match msg.guild_id {Some(_) => {} None => {return;}}
        let scanners = &self.db.get_scanners_by_disid(msg.guild_id.unwrap().to_string()).await.unwrap();
        for scanner in scanners {
            match scanner.scanner_backend {
                ScannerType::Pattern(ref pattern) => {
                    if pattern.is_match(&msg.content) {
                        msg.author.dm(&ctx.http, messages::block::blockedmessage(&msg.content, &scanner._id.to_hex())).await.unwrap();
                        msg.delete(&ctx.http).await.unwrap();
                    }
                }
                ScannerType::Word(ref word) => {
                    if word.is_match(&msg.content) {
                        msg.author.dm(&ctx.http, messages::block::blockedmessage(&msg.content, &scanner._id.to_hex())).await.unwrap();
                        msg.delete(&ctx.http).await.unwrap();
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
            Command::create_global_command(&ctx.http, commands::wonderful_command::register())
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
