use regex::Regex;

pub struct Pattern {
    regex: String,
    multiline: bool,
    case_insensitive: bool,
}


impl Pattern {
    pub fn new(regex: &str, multiline: bool, case_insensitive: bool) -> Pattern {
        Pattern {
            regex: regex.to_string(),
            multiline: multiline,
            case_insensitive: case_insensitive,
        }
    }

    pub fn compile(&self) -> Regex {
        let mut options = regex::RegexBuilder::new(&self.regex);
        options.multi_line(self.multiline);
        options.case_insensitive(self.case_insensitive);
        options.build().unwrap()
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.compile().is_match(text)
    }
}