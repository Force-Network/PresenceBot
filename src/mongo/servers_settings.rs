use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerSettings {
    pub _id: ObjectId,
    pub discord_id: String,
    pub log_channel: String,
}

impl ServerSettings {
    pub fn from_document(doc: mongodb::bson::Document) -> Result<Self, mongodb::error::Error> {
        Ok(ServerSettings {
            _id: doc.get_object_id("_id").unwrap(),
            discord_id: doc.get_str("discord_id").unwrap().to_string(),
            log_channel: doc.get_str("log_channel").unwrap().to_string(),
        })
    }

    pub fn to_document(&self) -> Document {
        doc! {
            "discord_id": self.discord_id.clone(),
            "log_channel": self.log_channel.clone(),
        }
    }
}