use mongodb::bson::doc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Punishment {
    Ban(Ban),
    Kick(Kick),
    Timeout(Timeout),
    No(NoPunishment)
}

impl Punishment {
    pub fn from_document(doc: mongodb::bson::Document) -> Result<Self, mongodb::error::Error> {
        Ok(match doc.get_str("type").unwrap() {
            "Ban" => Punishment::Ban(Ban {
                reason: doc.get_str("reason").unwrap().to_string(),
                duration: doc.get_i32("duration").unwrap(),
            }),
            "Kick" => Punishment::Kick(Kick {
                reason: doc.get_str("reason").unwrap().to_string(),
            }),
            "Timeout" => Punishment::Timeout(Timeout {
                duration: doc.get_i32("duration").unwrap(),
                reason: doc.get_str("reason").unwrap().to_string(),
            }),
            "No" => Punishment::No(NoPunishment {}),
            _ => panic!("Invalid punishment type"),
        })
    }

    pub fn to_document(&self) -> mongodb::bson::Document {
        let new_doc = match self {
            Punishment::Ban(b) => {
                doc! {
                    "type": "Ban",
                    "reason": b.reason.clone(),
                    "duration": b.duration,
                }
            }
            Punishment::Kick(k) => {
                doc! {
                    "type": "Kick",
                    "reason": k.reason.clone(),
                }
            }
            Punishment::Timeout(t) => {
                doc! {
                    "type": "Timeout",
                    "duration": t.duration,
                    "reason": t.reason.clone(),
                }
            },
            Punishment::No(_) => {
                doc! {
                    "type": "No",
                }
            }
        };
        new_doc
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Ban {
    pub reason: String,
    pub duration: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kick {
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timeout {
    pub duration: i32,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoPunishment {
}