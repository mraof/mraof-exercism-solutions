pub fn brackets_are_balanced(string: &str) -> bool {
    let mut unmatched = Vec::new();
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => unmatched.push(c),
            '}' => if unmatched.pop().map_or(true, |b| b != '{') { return false; },
            ']' => if unmatched.pop().map_or(true, |b| b != '[') { return false; },
            ')' => if unmatched.pop().map_or(true, |b| b != '(') { return false; },
            _ => ()
        }
    }
    unmatched.is_empty()
}
