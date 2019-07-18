use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None
    }
    let sum: u64 = (1..=(num / 2)).filter(|&n| (num % n) == 0).sum();
    Some(match sum.cmp(&num) {
        Ordering::Less => Classification::Deficient,
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
    })
}
