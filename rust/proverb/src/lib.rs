pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if list.is_empty() {
        return proverb
    }
    for i in 0..(list.len() - 1) {
        proverb += &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1])
    }
    proverb += &format!("And all for the want of a {}.", list[0]);
    proverb
}
