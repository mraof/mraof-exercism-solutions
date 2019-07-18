pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!"
    }
    let question = message.ends_with('?');
    let yelling = !message.chars().any(|c| c.is_lowercase()) && message.chars().any(|c| c.is_uppercase());
    match (question, yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever."
    }
}
