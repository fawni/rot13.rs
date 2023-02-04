/// Encode a value as rot13
pub trait Rot13 {
    /// Encode a value as rot13
    fn rot13(&self) -> String;
}

impl Rot13 for str {
    fn rot13(&self) -> String {
        self.chars().map(rot13).collect::<String>()
    }
}

/// Takes a character and encodes it as rot13
fn rot13(c: char) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    };

    if c.is_uppercase() {
        ((c as u8 + 13 - b'A') % 26 + b'A') as char
    } else {
        ((c as u8 + 13 - b'a') % 26 + b'a') as char
    }
}

#[cfg(test)]
mod tests {
    use crate::Rot13;

    static ROT13: &[(&str, &str); 5] = &[
        ("aha", "nun"),
        ("envy", "rail"),
        ("irk", "vex"),
        ("Hello, World!", "Uryyb, Jbeyq!"),
        ("beautiful.", "ornhgvshy."),
    ];

    #[test]
    fn test_rot13() {
        for (input, ciphered) in ROT13 {
            assert_eq!(input.rot13(), *ciphered);
        }
    }
}
