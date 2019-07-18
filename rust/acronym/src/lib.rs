pub fn abbreviate(phrase: &str) -> String {
    let r = regex::Regex::new(r"([a-z])([A-Z])").unwrap();
    r.replace_all(&phrase, "$1 $2")
        .split(&[' ', '-'][..])
        .filter_map(|word| match word.chars().next() {
            Some(c) if c.is_alphabetic() => Some(c),
            _ => None,
        })
        .collect::<String>()
        .to_uppercase()
}
