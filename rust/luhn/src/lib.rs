/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut count = 0;
    let mut sum = 0;
    for c in code.chars().rev().filter(|&c| c != ' ') {
        if let Some(mut d) = c.to_digit(10) {
            count += 1;
            if (count % 2) == 0 {
                d *= 2;
                if d > 9 {
                    d -= 9
                }
            }
            sum += d
        } else {
            return false
        }
    }
    count > 1 && (sum % 10) == 0
}
