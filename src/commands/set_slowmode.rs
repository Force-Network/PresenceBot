use std::sync::Arc;

use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, EditChannel};
use serenity::all::{Context, Permissions};
use serenity::model::application::Interaction;
use serenity::builder::{ CreateAutocompleteResponse};
use serenity::model::application::{ResolvedOption, ResolvedValue};

use crate::mongo::mongo_repo::MongoRepo;

pub async fn run<'a>(ctx: &'a Context, interaction: &'a Interaction, options: &[ResolvedOption<'_>], db: Arc<MongoRepo>) -> String {
    if !interaction.clone().as_command().unwrap().member.clone().unwrap().permissions.unwrap().manage_channels() {
        return "You need to be manager of channels to use this command".to_string()
    }
    if let Some(ResolvedOption { value: ResolvedValue::String(seconds), ..}) = options.first() {
        let seconds = seconds.parse::<u64>().unwrap();
        let seconds = if seconds > u16::MAX as u64 { u16::MAX } else { seconds as u16 };
        let _ = interaction.clone().as_command().unwrap().channel_id.edit(&ctx.http, Box::new(EditChannel::new()).rate_limit_per_user(seconds)).await;
        return format!("Successfully set slowmode to {} seconds", seconds)
    }
    "Hi".to_string()
}



pub fn register() -> CreateCommand {
    CreateCommand::new("slowmode")
        .description("Changes the slowmode")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "seconds", "How long the slowmode is")
                .required(true),
        )
}