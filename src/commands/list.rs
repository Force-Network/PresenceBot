use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter};
use std::sync::Arc;

use mongodb::options::FindOneAndUpdateOptions;
use serenity::all::{Context, Permissions};
use serenity::model::application::Interaction;
use serenity::builder::{CreateAutocompleteResponse};
use serenity::model::application::{ResolvedOption, ResolvedValue};

use crate::mongo::mongo_repo::MongoRepo;
use crate::mongo::scanner::{Scanner, ScannerType};
use crate::scanners::punishments::{self, Ban, Kick, Punishment, Timeout};
use crate::scanners::regex::Pattern;
use crate::utils::command_parser::convert_text_to_time_length;
use crate::utils::paging::{Book, ProceesedBook};
use crate::utils::send_log::send_log_new_filter;
use crate::DNR;

pub async fn run<'a>(ctx: &'a Context, interaction: &'a Interaction, options: &[ResolvedOption<'_>], db: Arc<MongoRepo>) -> String {
    let mut page = 1;
    if !interaction.clone().as_command().unwrap().member.clone().unwrap().permissions.unwrap().manage_guild() {
        return "You need to be an administrator to use this command".to_string()
    }
    if options.first().is_some() {
        if let Some(ResolvedOption { value: ResolvedValue::Integer(val), .. }) = options.first() {
            page = *val as i64;
        }
    }
    let filters = db.get_scanners_by_disid(interaction.clone().as_command().unwrap().guild_id.unwrap().to_string()).await.unwrap();
    let book = Book::new(filters);
    if page > book.get_page_count(10) as i64 {
        page = book.get_page_count(10 ) as i64;
    }
    let bookpage = book.get_page(page as i32, 10);

    let embed = CreateEmbed::default()
        .title("Filters")
        .fields(bookpage.iter().map(|filter| {
            let scanner = filter.scanner_backend.clone();
            let punishment = filter.punishment.clone();
            let scanner = match scanner {
                ScannerType::Pattern(pattern) => {
                    format!("Pattern: {}\nCase Insensitive: {}\nMultiline: {}", pattern.regex, pattern.case_insensitive, pattern.multiline)
                }
                ScannerType::Word(word) => {
                    format!("Word: {}\nCase Insensitive: {}", word.word, word.case_insensitive)
                }
            };
            let punishment = match punishment {
                Punishment::Ban(ban) => {
                    format!("Ban: {}\nDuration: {}", ban.reason, ban.duration)
                }
                Punishment::Kick(kick) => {
                    format!("Kick: {}", kick.reason)
                }
                Punishment::Timeout(timeout) => {
                    format!("Timeout: {}\nDuration: {}", timeout.reason, timeout.duration)
                }
                Punishment::No(_) => {
                    "No punishment".to_string()
                }
            };
            (format!("{}: {}", filter._id, scanner), punishment, false)
        }))
        .footer(CreateEmbedFooter::new(format!("Page {}/{}", page, book.get_page_count(10))));
    return DNR.to_string();
}


pub fn register() -> CreateCommand {
    CreateCommand::new("list")
        .description("list the filters")
    .add_option(CreateCommandOption::new(CommandOptionType::Integer, "Page", "Which page, defaults to 1.").required(false))
}