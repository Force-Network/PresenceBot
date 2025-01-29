use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, Interaction, ResolvedOption, ResolvedValue};
use std::sync::Arc;
use crate::MongoRepo;

pub fn register() -> CreateCommand {
    CreateCommand::new("setup").description("A setup command").add_option(CreateCommandOption::new(CommandOptionType::Channel, "channel", "The channel to send logs to").required(true))
}

pub async fn run<'a>(ctx: &'a Context, interaction: &'a Interaction, options: &[ResolvedOption<'_>], db: Arc<MongoRepo>) -> String {
    if let Some(ResolvedOption { value: ResolvedValue::Channel(channel), ..}) = options.first() {
        let mut settings = db.get_settings_by_disid(interaction.clone().as_command().unwrap().guild_id.unwrap().to_string()).await.unwrap();
        settings.log_channel = channel.id.to_string();
        db.update_settings(settings).await.unwrap();
        return "Successfully set log channel".to_string();
    }
    "Failed to set log channel".to_string()
}