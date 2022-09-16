use std::hash::{Hash, Hasher};

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

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.byte == other.byte
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.byte.hash(state)
    }
}
