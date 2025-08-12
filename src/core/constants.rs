use once_cell::sync::Lazy;
use regex::Regex;

pub static AUTHOR: &str = "liblaf";

pub static PATTERN_START: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&(regex::escape("tangerine-start") + r#":\s*(?P<name>\S+)\s*(?P<context>\{.*\})?"#))
        .unwrap()
});

pub static PATTERN_END: Lazy<Regex> =
    Lazy::new(|| Regex::new(&regex::escape("tangerine-end")).unwrap());
