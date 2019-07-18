pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let mut chars = source.chars();
    let mut vec = vec![(1, chars.next().unwrap())];
    for c in chars {
        let last = vec.len() - 1;
        if c != vec[last].1 {
            vec.push((1, c));
        } else {
            vec[last].0 += 1;
        }
    }

    vec.into_iter().map(|(count, c)| {
        if count > 1 {
            format!("{}{}", count, c)
        } else {
            c.to_string()
        }
    }).collect()
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut num_string = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            num_string.push(c)
        } else {
            let num = num_string.parse::<u32>().unwrap_or(1);
            for _ in 0..num {
                decoded.push(c)
            }
            num_string.clear()
        }
    }
    decoded
}
