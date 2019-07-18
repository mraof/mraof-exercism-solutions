pub fn raindrops(n: u32) -> String {
    let mut output = String::new();
    if n % 3 == 0 {
        output += "Pling"
    }
    if n % 5 == 0 {
        output += "Plang"
    }
    if n % 7 == 0 {
        output += "Plong"
    }
    if output.is_empty() {
        format!("{}", n)
    } else {
        output
    }
}
