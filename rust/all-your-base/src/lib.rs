#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    let mut num = 0;
    for &d in number {
        if d >= from_base {
            return Err(Error::InvalidDigit(d));
        }
        num *= from_base;
        num += d;
    }
    let mut vec = Vec::new();
    while num > 0 {
        vec.insert(0, num % to_base);
        num /= to_base;
    }
    Ok(vec)
}
