use crate::parse::token::Token;

pub const CHARSET: [Token; 1] = [Token::fixed(b'a', 'a', "a")];

#[cfg(test)]
mod tests {
    use ::std::collections::HashSet;
    use ::std::hash::Hash;

    use super::*;

    fn check_prop_unique<T: Eq + Hash>(getter: fn(Token) -> T) {
        let mut seen = HashSet::new();
        for c in CHARSET {
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
