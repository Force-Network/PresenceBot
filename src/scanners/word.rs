use serde::Deserialize;
use serde::Serialize;

use super::general::convert_all_unicode_to_ascii;
use super::general::ScannerBackend;

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub word: String,
    pub case_insensitive: bool,
    pub remove_unicode: bool,
}

impl Word {
    pub fn new(word: &str, case_insensitive: bool, remove_unicode: bool) -> Word {
        Word {
            word: word.to_string(),
            case_insensitive: case_insensitive,
            remove_unicode: remove_unicode,
        }
    }

    pub fn is_match(&self, text: &str) -> bool {
        let text = if self.remove_unicode {
            convert_all_unicode_to_ascii(text)
        } else {
            text.to_string()
        };

        let word = if self.remove_unicode {
            convert_all_unicode_to_ascii(&self.word)
        } else {
            self.word.to_string()
        };

        if self.case_insensitive {
            text.to_lowercase().contains(&word.to_lowercase())
        } else {
            text.contains(&word)
        }
    }

    pub fn change_settings(&mut self, case_insensitive: bool, remove_unicode: bool) {
        self.case_insensitive = case_insensitive;
        self.remove_unicode = remove_unicode;
    }

    pub fn from_document(doc: mongodb::bson::Document) -> Result<Self, mongodb::error::Error> {
        Ok(Word {
            word: doc.get_str("word").unwrap().to_string(),
            remove_unicode: doc.get_bool("remove_unicode").unwrap(),
            case_insensitive: doc.get_bool("case_insensitive").unwrap(),
        })
    }

}

impl ScannerBackend for Word {
    fn scan(&self, text: &str) -> bool {
        self.is_match(text)
    }
}
