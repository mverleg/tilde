
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Token appears alone, or followed by modifiers.
    VariableOpen,
    /// Token is followed by one other token of any type, and then optional modifiers.
    FixedOpen,
    /// Affects the previous opener, or the whole program if there is no preceding opener.
    Modifier,
}

#[derive(Debug, Clone)]
pub struct Token {
    byte: u8,
    chr: char,
    long: &'static str,
    typ: TokenType,
}

impl Token {
    pub const fn new(byte: u8, chr: char, long: &'static str, typ: TokenType) -> Self {
        Token {
            byte,
            chr,
            long,
            typ
        }
    }

    pub const fn var(byte: u8, chr: char, long: &'static str) -> Self {
        Token::new(byte, chr, long, TokenType::VariableOpen)
    }

    pub const fn fixed(byte: u8, chr: char, long: &'static str) -> Self {
        Token::new(byte, chr, long, TokenType::FixedOpen)
    }

    pub const fn modi(byte: u8, chr: char, long: &'static str) -> Self {
        Token::new(byte, chr, long, TokenType::Modifier)
    }
}

pub const CHARSET: [Token; 1] = [
    Token::fixed(b'a', 'a', "a"),
];

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

