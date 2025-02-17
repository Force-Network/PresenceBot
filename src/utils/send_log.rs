use std::sync::Arc;

use serenity::all::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, Message, User};

use crate::mongo::mongo_repo::MongoRepo;

pub async fn send_log_block(blocked_message: Message, user: User, filter: String, ctx: serenity::client::Context, db: Arc<MongoRepo>) {
    let settings = db.get_settings_by_disid(blocked_message.guild_id.unwrap().to_string()).await.unwrap();
    if settings.log_channel.is_empty() {
        return;
    }
    let log_channel = settings.log_channel.parse::<u64>().unwrap();
    let log_channel: serenity::model::channel::GuildChannel = ctx.http.get_channel(log_channel.into()).await.unwrap().guild().unwrap();
    let embed = CreateEmbed::default()
        .title("Blocked message")
        .description(format!("User: {} \n Message: ```{}```", user.name, blocked_message.content))
        .author(CreateEmbedAuthor::new(user.clone().name).icon_url(user.avatar_url().unwrap_or_default()))
        .footer(CreateEmbedFooter::new(format!("Filter: {}", filter)));
    let message = log_channel.id.send_message(&ctx.http, CreateMessage::new().add_embed(embed)).await.unwrap();
}

pub async fn send_log_new_filter(filter: String, r#type: String, creater: User, guild_id: String, ctx: serenity::client::Context, db: Arc<MongoRepo>) {
    let settings = db.get_settings_by_disid(guild_id.clone()).await.unwrap();
    if settings.log_channel.is_empty() {
        return;
    }
    let log_channel = settings.log_channel.parse::<u64>().unwrap();
    let log_channel: serenity::model::channel::GuildChannel = ctx.http.get_channel(log_channel.into()).await.unwrap().guild().unwrap();
    let embed = CreateEmbed::default()
        .title("New filter")
        .description(format!("Filter: {}\n Type: {}", &filter, &r#type))
        .author(CreateEmbedAuthor::new(creater.clone().name).icon_url(creater.avatar_url().unwrap_or_default()));
    let _ = log_channel.id.send_message(&ctx.http, CreateMessage::new().add_embed(embed)).await.unwrap();
}