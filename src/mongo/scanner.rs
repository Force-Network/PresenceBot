use std::error::Error;

use crate::scanners::punishments::{self, NoPunishment};
use crate::scanners::{general::ScannerBackend, punishments::Punishment};
use crate::scanners::regex::Pattern;
use crate::scanners::word::Word;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scanner {
    pub _id: ObjectId,
    pub discord_id: String,
    pub scanner_backend: ScannerType,
    pub punishment: Punishment
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum ScannerType {
    Pattern(Pattern),
    Word(Word),
}

impl Scanner {
    pub fn from_document(doc: mongodb::bson::Document) -> Result<Self, mongodb::error::Error> {
        Ok(Scanner {
            _id: doc.get_object_id("_id").unwrap(),
            discord_id: doc.get_str("discord_id").unwrap().to_string(),
            scanner_backend: match doc.get_document("scanner_backend").unwrap().get_str("type").unwrap() {
                "Pattern" => {
                    ScannerType::Pattern(Pattern::from_document(doc.get_document("scanner_backend").unwrap().clone()).unwrap())
                }
                "Word" => {
                    ScannerType::Word(Word::from_document(doc.get_document("scanner_backend").unwrap().clone()).unwrap())
                }
                _ => panic!("Invalid scanner type"),
            },
            punishment: punishments::Punishment::from_document(doc.get_document("punishment").unwrap_or(&doc! {"type":"No"}).clone()).unwrap(),
        })
    }

    pub fn to_document(&self) -> Document {
        let new_doc = doc! {
            "discord_id": self.discord_id.clone(),
            "scanner_backend": match &self.scanner_backend {
                ScannerType::Pattern(p) => {
                    doc! {
                        "type": "Pattern",
                        "regex": p.regex.clone(),
                        "multiline": p.multiline,
                        "case_insensitive": p.case_insensitive,
                    }
                }
                ScannerType::Word(w) => {
                    doc! {
                        "type": "Word",
                        "word": w.word.clone(),
                        "case_insensitive": w.case_insensitive,
                        "remove_unicode": w.remove_unicode,
                    }
                }
            },
            "punishment": self.punishment.to_document(),
        };
        new_doc
    }
}

