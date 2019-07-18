pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut count = 0;
        let mut sum = 0;
        for c in self.code.chars().rev().filter(|&c| c != ' ') {
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
                return false;
            }
        }
        count > 1 && (sum % 10) == 0
    }
}

impl<S> From<S> for Luhn
where
    S: ToString,
{
    fn from(input: S) -> Self {
        Luhn {
            code: input.to_string(),
        }
    }
}
