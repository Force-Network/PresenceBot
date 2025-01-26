use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};

pub fn blockedmessage(msgcontent: &String, rule: &String) -> CreateMessage {
    CreateMessage::new().add_embed(CreateEmbed::new().title("Your message was deleted").description(format!("Your message was detected to violate server rules! If you think this is error please contact server staff. \nYou sent the message:\n```{}```", msgcontent)).footer(CreateEmbedFooter::new(format!("Rule: {}", rule))))
}