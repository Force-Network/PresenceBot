//
//   Genaral operations that many scanners may need to do
//

use decancer;

pub fn convert_all_unicode_to_ascii(text: &str) -> String {
    decancer::cure(text, decancer::Options::default().retain_capitalization()).unwrap().to_string()
}