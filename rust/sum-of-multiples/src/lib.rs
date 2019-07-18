use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();
    for &factor in factors {
        if factor == 0 {
            set.insert(0);
        } else {
            set.extend((factor..limit).step_by(factor as usize))
        }
    }
    set.into_iter().sum()
}
