use std::sync::Arc;

use mongodb::options::FindOneAndUpdateOptions;
use serenity::all::{Context};
use serenity::model::application::Interaction;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateAutocompleteResponse};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::mongo::mongo_repo::MongoRepo;
use crate::mongo::scanner::{Scanner, ScannerType};
use crate::scanners::punishments::{self, Ban, Kick, Punishment, Timeout};
use crate::scanners::regex::Pattern;
use crate::utils::command_parser::convert_text_to_time_length;

pub async fn run<'a>(ctx: &'a Context, interaction: &'a Interaction, options: &[ResolvedOption<'_>], db: Arc<MongoRepo>) -> String {
    if let Some(ResolvedOption { value: ResolvedValue::String(user), ..}) = options.first() {
        let case_insensitive: bool = if let Some(ResolvedOption { value: ResolvedValue::Boolean(case_insensitive), ..}) = options.get(2) {
            *case_insensitive
        } else {
            false
        };

        let multiline: bool = if let Some(ResolvedOption { value: ResolvedValue::Boolean(multiline), ..}) = options.get(3) {
            *multiline
        } else {
            false
        };
        let punishment: String = if let Some(ResolvedOption { value: ResolvedValue::String(punishment), ..}) = options.get(1) {
            punishment.to_string()
        } else {
            return "Punishment is required".to_string()
        };
        let punishment = match punishment.as_str() {
            s if s.contains("ban") => Punishment::Ban(Ban {reason: "Ban via punishment of rule $RULE".to_string(), duration: convert_text_to_time_length(punishment.split("ban ").nth(1).unwrap_or("")) as i32 }),
            s if s.contains("kick") => Punishment::Kick(Kick {reason: "Kick via punishment of rule $RULE".to_string()}),
            s if s.contains("mute") => Punishment::Timeout(Timeout {reason: "Mute via punishment of rule $RULE".to_string(), duration: convert_text_to_time_length(punishment.split("mute ").nth(1).unwrap_or("")) as i32 }),
            _ => Punishment::No(punishments::NoPunishment {}),
        };
        let pattern = Pattern::new(&user, multiline, case_insensitive);
                let scanner = Scanner {
                    _id: mongodb::bson::oid::ObjectId::new(),
                    discord_id: interaction.clone().as_command().unwrap().guild_id.unwrap().to_string(),
                    scanner_backend: ScannerType::Pattern(pattern),
                    punishment: punishment,
                };
                match db.create_scanner(scanner).await {
                    Ok(_) => return "Successfully added regex rule".to_string(),
                    Err(_) => return "Failed to add regex rule".to_string()
                }
}
    "Your inputs were wrong, please recheck them, no clue how you would ever see this, this would only happen if you REALLY messed something, and discord should make this impostable.".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("addregexrule")
        .description("Get a regex rule")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "regexpattern", "A regex pattern")
                .required(true),
        )
        .add_option(CreateCommandOption::new(CommandOptionType::String, "punishment", "Punishment to apply").required(true).set_autocomplete(false))
        .add_option(CreateCommandOption::new(CommandOptionType::Boolean, "caseinsensitive", "Case insensitive matching").required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::Boolean, "multiline", "Multiline matching").required(false))
}