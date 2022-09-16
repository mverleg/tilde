use crate::parse::Token;

pub static TOKENSET: [Token; 5] = [
    Token::fixed(b'a', 'a', "a"),
    Token::modi(252, '@', "harder"),
    Token::modi(253, '#', "faster"),
    Token::modi(254, '$', "stronger"),
    Token::modi(255, '%', "larger"),
];

#[cfg(test)]
mod tests {
    use ::std::collections::HashSet;
    use ::std::hash::Hash;

    use super::*;

    fn check_prop_unique<T: Eq + Hash>(getter: fn(&Token) -> T) {
        let mut seen = HashSet::new();
        for c in &TOKENSET {
            assert!(seen.insert(getter(c)))
        }
        assert_eq!(seen.len(), u8::MAX as usize)
    }

    #[test]
    fn unique_bytes() {
        check_prop_unique(|c| c.byte)
    }

    #[test]
    fn unique_char() {
        check_prop_unique(|c| c.chr)
    }

    #[test]
    fn unique_long_identifier() {
        check_prop_unique(|c| c.long)
    }
}
