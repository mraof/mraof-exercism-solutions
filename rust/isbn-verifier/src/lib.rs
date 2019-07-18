/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut chars = isbn.chars();
    let mut d = Vec::with_capacity(9);
    while let Some(c) = chars.next() {
        if let Some(digit) = c.to_digit(10) {
            d.push(digit);
            if d.len() == 9 {
                break;
            }
        } else if c != '-' {
            return false;
        }
    }
    if d.len() != 9 {
        return false;
    }

    let mut check = None;
    while let Some(c) = chars.next() {
        if c != '-' {
            if c == 'X' {
                check = Some(10)
            } else {
                check = c.to_digit(10)
            }
            break;
        }
    }

    if chars.next() != None {
        return false;
    }

    if let Some(check) = check {
        ((d[0] * 10 + d[1] * 9 + d[2] * 8 + d[3] * 7 + d[4] * 6 + d[5] * 5 + d[6] * 4 + d[7] * 3 + d[8] * 2 + check) % 11) == 0
    } else {
        false
    }
}
