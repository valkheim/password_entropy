fn has_ascii_digits(s: &str) -> bool {
    s.chars().any(|c| char::is_ascii_digit(&c))
}

fn has_ascii_lowercase(s: &str) -> bool {
    s.chars().any(|c| char::is_ascii_lowercase(&c))
}

fn has_ascii_uppercase(s: &str) -> bool {
    s.chars().any(|c| char::is_ascii_uppercase(&c))
}

fn has_ascii_special_characters(s: &str) -> bool {
    s.chars()
        .any(|c| char::is_ascii_punctuation(&c) || c == ' ')
}

fn ascii_size(s: &str) -> u32 {
    type PoolChecker = fn(&str) -> bool;
    let a: [(PoolChecker, u32); 4] = [
        (has_ascii_digits, 10),
        (has_ascii_lowercase, 26),
        (has_ascii_uppercase, 26),
        (has_ascii_special_characters, 33),
    ];
    a.iter()
        .filter(|(pool_check, _)| pool_check(s))
        .map(|(_, pool_size)| pool_size)
        .sum()
}

pub fn size(s: &str) -> u32 {
    ascii_size(s)
}

#[cfg(test)]
mod tests {
    use crate::pool::{
        ascii_size, has_ascii_digits, has_ascii_lowercase, has_ascii_special_characters,
        has_ascii_uppercase,
    };

    #[test]
    fn test_has_ascii_digits() {
        assert!(has_ascii_digits("0"));
        assert!(has_ascii_digits("abc0def"));
        assert!(has_ascii_digits(" 21 "));

        assert!(!has_ascii_digits(""));
        assert!(!has_ascii_digits("abc"));
    }

    #[test]
    fn test_has_ascii_lowercase() {
        assert!(has_ascii_lowercase("abc"));

        assert!(!has_ascii_lowercase(""));
        assert!(!has_ascii_lowercase("0"));
    }

    #[test]
    fn test_has_ascii_uppercase() {
        assert!(has_ascii_uppercase("ABC"));

        assert!(!has_ascii_uppercase(""));
        assert!(!has_ascii_uppercase("0"));
        assert!(!has_ascii_uppercase("abc"));
    }

    #[test]
    fn test_has_ascii_special_characters() {
        assert!(has_ascii_special_characters("!"));
        assert!(has_ascii_special_characters("\""));
        assert!(has_ascii_special_characters("#"));
        assert!(has_ascii_special_characters("$"));
        assert!(has_ascii_special_characters("%"));
        assert!(has_ascii_special_characters("&"));
        assert!(has_ascii_special_characters("'"));
        assert!(has_ascii_special_characters("("));
        assert!(has_ascii_special_characters(")"));
        assert!(has_ascii_special_characters("*"));
        assert!(has_ascii_special_characters("+"));
        assert!(has_ascii_special_characters(","));
        assert!(has_ascii_special_characters("-"));
        assert!(has_ascii_special_characters("."));
        assert!(has_ascii_special_characters("/"));
        assert!(has_ascii_special_characters(":"));
        assert!(has_ascii_special_characters(";"));
        assert!(has_ascii_special_characters("<"));
        assert!(has_ascii_special_characters("="));
        assert!(has_ascii_special_characters(">"));
        assert!(has_ascii_special_characters("?"));
        assert!(has_ascii_special_characters("@"));
        assert!(has_ascii_special_characters("["));
        assert!(has_ascii_special_characters("\\"));
        assert!(has_ascii_special_characters("]"));
        assert!(has_ascii_special_characters("^"));
        assert!(has_ascii_special_characters("_"));
        assert!(has_ascii_special_characters("`"));
        assert!(has_ascii_special_characters("{"));
        assert!(has_ascii_special_characters("|"));
        assert!(has_ascii_special_characters("}"));
        assert!(has_ascii_special_characters("~"));
        assert!(has_ascii_special_characters(" "));
    }

    #[test]
    fn test_ascii_size() {
        assert_eq!(ascii_size("0"), 10);
        assert_eq!(ascii_size("password"), 26);
        assert_eq!(ascii_size("Password"), 26 * 2);
        assert_eq!(ascii_size("Password$"), 26 * 2 + 33);
        assert_eq!(ascii_size("Password123"), 26 * 2 + 10);
        assert_eq!(ascii_size("Password123$"), 26 * 2 + 10 + 33);
    }
}
