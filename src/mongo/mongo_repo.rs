use std::collections::HashMap;
use std::sync::Arc;
use std::{env, str};
extern crate dotenv;

use dotenv::dotenv;
use moka::future::Cache;
use mongodb::action::InsertOne;
use mongodb::{Client, Collection, Cursor};
use serenity::cache;
use serenity::futures::stream::Collect;
use serenity::futures::StreamExt;
use serenity::model::id;

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

use super::scanner::Scanner;
use super::servers_settings::{self, ServerSettings};

pub struct MongoRepo {
    cache: Arc<Cache<String, String>>,
    col_scanners: Collection<Document>,
    col_settings: Collection<Document>
}

impl MongoRepo {
    pub async fn init(cache: Arc<Cache<String, String>>) -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("MONGOURI must be set in .env file");
        let client = Client::with_uri_str(&uri).await.expect("Failed to initialize MongoDB client");
        let db = client.database("PresenceBot");
        let col_scanners: Collection<Document> = db.collection("scanner");
        let col_settings: Collection<Document> = db.collection("settings");
        MongoRepo {
            cache,
            col_scanners,
            col_settings
        }
    }

    pub async fn create_scanner(&self, new_scanner: Scanner) -> Result<InsertOneResult, Error> {
        let result = self.col_scanners.insert_one(new_scanner.to_document()).await;
        self.cache.remove(&format!("{}_scanners", new_scanner.discord_id)).await;
        result
    }

    pub async fn get_scanners_by_disid(&self, discord_id: String) -> Result<Vec<Scanner>, Error> {
        let key = format!("{}_scanners", discord_id.clone());
        if self.cache.contains_key(&key) {
            let scanners = self.cache.get(&key).await.unwrap();
            return Ok(serde_json::from_str(&scanners).unwrap());
        }
        let mut cursor: Cursor<Document> = self.col_scanners.find(doc! {"discord_id": discord_id.clone()}).await?;
        let mut scanners: Vec<Scanner> = Vec::new();
        while let Some(doc) = cursor.next().await {
            scanners.push(Scanner::from_document(doc?)?);
        }
        let json = serde_json::to_string(&scanners);
        self.cache.insert(key.clone(), json.unwrap()).await;
        if !self.cache.contains_key(&key) {
            println!("Cache insert failed");
        }
        Ok(scanners)
    }

    pub async fn create_server_settings(&self, new_settings: ServerSettings) -> Result<InsertOneResult, Error> {
        let result = self.col_settings.insert_one(new_settings.to_document()).await;
        result
    }

    pub async fn get_settings_by_disid(&self, discord_id: String) -> Result<ServerSettings, Error> {
        // make sure the settings exist
        if self.col_settings.find_one(doc! {"discord_id": discord_id.clone()}).await.unwrap().is_none() {
            let settings = ServerSettings {_id: ObjectId::new(), discord_id: discord_id.clone(), log_channel: "".to_string()};
            self.create_server_settings(settings).await.unwrap();
        }
        let key = format!("{}_settings", discord_id.clone());
        if self.cache.contains_key(&key) {
            let settings = self.cache.get(&key).await.unwrap();
            return Ok(serde_json::from_str(&settings).unwrap());
        }
        let settingdoc = self.col_settings.find_one(doc! {"discord_id": discord_id.clone()}).await.unwrap().unwrap_or(ServerSettings {_id: ObjectId::new(), discord_id: discord_id.clone(), log_channel: "".to_string()}.to_document());
        let server_settings = ServerSettings::from_document(settingdoc)?;
        let json = serde_json::to_string(&server_settings).unwrap();
        self.cache.insert(key.clone(), json).await;
        Ok(server_settings)
    }

    pub async fn update_settings(&self, settings: ServerSettings) -> Result<UpdateResult, Error> {
        let result = self.col_settings.update_one(doc! {"discord_id": settings.discord_id.clone()}, doc! {"$set": settings.to_document()}).await;
        self.cache.remove(&format!("{}_settings", settings.discord_id)).await;
        result
    }
}