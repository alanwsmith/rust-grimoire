// cargo add regex

use regex::Regex;

fn scrub_url_path(source: String) -> String {
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(&source, "-").to_lowercase()
}

fn main() {
    let source = String::from("this Is  123 Some Text");
    let expected = String::from("this-is-123-some-text");
    let result = scrub_url_path(source);
    assert_eq!(expected, result);
}

