use crate::scanners::general::ScannerBackend;
use mongodb::bson::Document;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub regex: String,
    pub multiline: bool,
    pub case_insensitive: bool,
}

impl Pattern {
    pub fn new(regex: &str, multiline: bool, case_insensitive: bool) -> Pattern {
        Pattern {
            regex: regex.to_string(),
            multiline: multiline,
            case_insensitive: case_insensitive,
        }
    }

    fn compile(&self) -> Regex {
        let mut options = regex::RegexBuilder::new(&self.regex);
        options.multi_line(self.multiline);
        options.case_insensitive(self.case_insensitive);
        options.build().unwrap()
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.compile().is_match(text)
    }

    pub fn change_settings(&mut self, multiline: bool, case_insensitive: bool) {
        self.multiline = multiline;
        self.case_insensitive = case_insensitive;
    }

    pub fn from_document(doc: Document) -> Result<Self, mongodb::error::Error> {
        Ok(Pattern {
            regex: doc.get_str("regex").unwrap().to_string(),

            multiline: doc.get_bool("multiline").unwrap(),

            case_insensitive: doc.get_bool("case_insensitive").unwrap(),
        })
    }
}

impl ScannerBackend for Pattern {
    fn scan(&self, text: &str) -> bool {
        self.is_match(text)
    }
}
