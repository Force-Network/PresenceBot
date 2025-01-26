use std::collections::HashMap;
use std::{env, str};
extern crate dotenv;

use dotenv::dotenv;
use mongodb::{Client, Collection, Cursor};
use serenity::futures::StreamExt;
use serenity::model::id;

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

use super::scanner::Scanner;

pub struct MongoRepo {
    col_scanners: Collection<Document>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("MONGOURI must be set in .env file");
        let client = Client::with_uri_str(&uri).await.expect("Failed to initialize MongoDB client");
        let db = client.database("PresenceBot");
        let col_scanners: Collection<Document> = db.collection("scanner");
        MongoRepo {
            col_scanners,
        }
    }

    pub async fn create_scanner(&self, new_scanner: Scanner) -> Result<InsertOneResult, Error> {
        let result = self.col_scanners.insert_one(new_scanner.to_document()).await;
        result
    }

    pub async fn get_scanners_by_disid(&self, discord_id: String) -> Result<Vec<Scanner>, Error> {
        let mut cursor: Cursor<Document> = self.col_scanners.find(doc! {"discord_id": discord_id}).await?;
        let mut scanners: Vec<Scanner> = Vec::new();
        while let Some(doc) = cursor.next().await {
            scanners.push(Scanner::from_document(doc?)?);
        }
        Ok(scanners)
    }

}