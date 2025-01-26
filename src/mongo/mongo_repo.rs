use std::collections::HashMap;
use std::sync::Arc;
use std::{env, str};
extern crate dotenv;

use dotenv::dotenv;
use moka::future::Cache;
use mongodb::{Client, Collection, Cursor};
use serenity::cache;
use serenity::futures::StreamExt;
use serenity::model::id;

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

use super::scanner::Scanner;

pub struct MongoRepo {
    cache: Arc<Cache<String, String>>,
    col_scanners: Collection<Document>,
}

impl MongoRepo {
    pub async fn init(cache: Arc<Cache<String, String>>) -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("MONGOURI must be set in .env file");
        let client = Client::with_uri_str(&uri).await.expect("Failed to initialize MongoDB client");
        let db = client.database("PresenceBot");
        let col_scanners: Collection<Document> = db.collection("scanner");
        MongoRepo {
            cache,
            col_scanners,
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
            println!("Cache hit");
            let scanners = self.cache.get(&key).await.unwrap();
            return Ok(serde_json::from_str(&scanners).unwrap());
        } else {
            println!("Cache miss");
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
}