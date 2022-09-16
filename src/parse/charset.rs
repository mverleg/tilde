
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
    long: String,
    typ: TokenType,
}

impl Token {
    pub const fn new(byte: u8, chr: char, long: impl Into<String>, typ: TokenType) -> Self {
        Token {
            byte,
            chr,
            long: long.into(),
            typ
        }
    }

    pub const fn var(byte: u8, chr: char, long: impl Into<String>) -> Self {
        Token::new(byte, chr, long, TokenType::VariableOpen)
    }

    pub const fn fixed(byte: u8, chr: char, long: impl Into<String>) -> Self {
        Token::new(byte, chr, long, TokenType::FixedOpen)
    }

    pub const fn modi(byte: u8, chr: char, long: impl Into<String>) -> Self {
        Token::new(byte, chr, long, TokenType::Modifier)
    }
}

pub const CHARSET: [Token; 1] = [
    Token::fixed(b'a', 'a', "a"),
];

#[cfg(test)]
mod tests {
    use ::std::collections::HashSet;

    use super::*;

    #[test]
    fn unique_bytes() {
        let mut seen = HashSet::new();
        for c in CHARSET {
            assert!(seen.insert(c.byte))
        }
        assert_eq!(seen.len(), u8::MAX)
    }

    #[test]
    fn unique_char() {
        let mut seen = HashSet::new();
        for c in CHARSET {
            assert!(seen.insert(c.chr))
        }
        assert_eq!(seen.len(), u8::MAX)
    }

    #[test]
    fn unique_long_identifier() {
        let mut seen = HashSet::new();
        for c in CHARSET {
            assert!(seen.insert(c.long))
        }
        assert_eq!(seen.len(), u8::MAX)
    }
}

