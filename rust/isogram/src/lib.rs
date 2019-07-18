use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();
    for c in candidate.to_lowercase().chars() {
        if c.is_alphanumeric() && !letters.insert(c) {
            return false
        }
    }
    true
}
