#[cfg(test)]
mod tests {
    use crate::scanners::regex::Pattern;

    #[test]
    fn regex_single_line_match() {
        let pattern = Pattern::new(r"^\d{3}-\d{3}-\d{4}$", false, false);
        assert_eq!(pattern.is_match("123-456-7890"), true);
    }

    #[test]
    fn regex_single_line_no_match() {
        let pattern = Pattern::new(r"^\d{3}-\d{3}-\d{4}$", false, false);
        assert_eq!(pattern.is_match("123-456-789"), false);
    }

    #[test]
    fn unicode_to_ascii_lookalike() {
        let text = "Hello, 𝕊𝕙𝕚 𝕁𝕚𝕖!";
        assert_eq!(
            crate::scanners::general::convert_all_unicode_to_ascii(text),
            "Hello, shi jie!"
        );
    }

    #[test]
    fn unicode_to_ascii_no_lookalike() {
        let text = "Hello, world!";
        assert_eq!(
            crate::scanners::general::convert_all_unicode_to_ascii(text),
            "Hello, world!"
        );
    }

    #[test]
    fn unicode_to_ascii_and_regex() {
        let text = "Hello, 𝕊𝕙𝕚 𝕁𝕚𝕖!";
        let text = crate::scanners::general::convert_all_unicode_to_ascii(text);
        let pattern = Pattern::new(r"^\w{5}, \w{3} \w{3}!$", false, false);
        assert_eq!(pattern.is_match(&text), true);
    }
}
