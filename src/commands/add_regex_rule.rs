use std::sync::Arc;

use mongodb::options::FindOneAndUpdateOptions;
use serenity::all::{Context};
use serenity::model::application::Interaction;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::mongo::mongo_repo::MongoRepo;
use crate::mongo::scanner::{Scanner, ScannerType};
use crate::scanners::regex::Pattern;

pub async fn run<'a>(ctx: &'a Context, interaction: &'a Interaction, options: &[ResolvedOption<'_>], db: Arc<MongoRepo>) -> String {
    if let Some(ResolvedOption { value: ResolvedValue::String(user), ..}) = options.first() {
        let case_insensitive: bool = if let Some(ResolvedOption { value: ResolvedValue::Boolean(case_insensitive), ..}) = options.get(1) {
            *case_insensitive
        } else {
            false
        };

        let multiline: bool = if let Some(ResolvedOption { value: ResolvedValue::Boolean(multiline), ..}) = options.get(2) {
            *multiline
        } else {
            false
        };
        let pattern = Pattern::new(&user, multiline, case_insensitive);
                let scanner = Scanner {
                    _id: mongodb::bson::oid::ObjectId::new(),
                    discord_id: interaction.clone().as_command().unwrap().guild_id.unwrap().to_string(),
                    case_insensitive: case_insensitive,
                    remove_unicode: false,
                    scanner_backend: ScannerType::Pattern(pattern),
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
        .add_option(CreateCommandOption::new(CommandOptionType::Boolean, "caseinsensitive", "Case insensitive matching").required(false))
        .add_option(CreateCommandOption::new(CommandOptionType::Boolean, "multiline", "Multiline matching").required(false))
}
