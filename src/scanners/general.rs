//
//   Genaral operations that many scanners may need to do
//
use decancer;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub fn convert_all_unicode_to_ascii(text: &str) -> String {
    decancer::cure(text, decancer::Options::default().retain_capitalization())
        .unwrap()
        .to_string()
}

pub trait ScannerBackend: Debug + Deserialize<'static> {
    fn scan(&self, text: &str) -> bool; // Scan items
}
