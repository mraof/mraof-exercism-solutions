use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let l = letters(&word);
    possible_anagrams.iter().filter(|possible| {
        let possible = possible.to_lowercase();
        possible != word && l == letters(&possible)
    }).copied().collect()
}

fn letters(word: &str) -> Vec<char> {
    let mut letters: Vec<char> = word.chars().collect();
    letters.sort();
    letters
}
