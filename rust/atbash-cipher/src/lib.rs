/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let plain = plain.to_lowercase();
    let encoded = plain
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c >= 'a' && c <= 'z' {
                char::from(b'z' + b'a' - c as u8)
            } else {
                c
            }
        })
        .collect::<Vec<char>>();
    encoded
        .chunks(5)
        .map(|chunk: &[char]| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| {
            if c == ' ' {
                None
            } else if c >= 'a' && c <= 'z' {
                Some(char::from(b'z' + b'a' - c as u8))
            } else {
                Some(c)
            }
        })
        .collect()
}
