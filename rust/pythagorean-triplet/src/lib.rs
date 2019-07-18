use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    //this solution sucks but performance doesn't matter enough for me to figure out a better one
    let mut set = HashSet::new();
    for a in 0..(sum / 3) {
        for b in a..(sum / 2) {
            let c2 = a*a + b*b;
            let c = (c2 as f64).sqrt() as u32;
            if (c*c) == c2 && (a + b + c) == sum {
                set.insert([a, b, c]);
            }
        }
    }
    set
}
