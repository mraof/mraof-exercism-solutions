use std::collections::{BTreeSet, HashMap, HashSet};
use std::iter::FromIterator;

//There are more efficient ways to do this
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    //this works because == is always last
    let (addends, sum) = {
        let mut split: Vec<_> = input.split(&[' ', '+', '='][..]).filter(|string| !string.is_empty()).collect();
        let sum = split.pop().unwrap();
        (split, sum)
    };
    println!("{:?} {}", addends, sum);
    unimplemented!()
}
