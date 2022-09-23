use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    /// Special token that starts a number or text literal.
    Literal,
    /// Token appears alone, or followed by modifiers.
    VariableOpen,
    /// Token is followed by one other token of any type, and then optional modifiers.
    FixedOpen,
    /// Affects the previous opener, or the whole program if there is no preceding opener.
    Modifier,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        use TokenType::*;
        match (self, other) {
            (VariableOpen, VariableOpen) => true,
            (VariableOpen, FixedOpen) => false,
            (VariableOpen, Modifier) => false,
            (FixedOpen, VariableOpen) => false,
            (FixedOpen, FixedOpen) => true,
            (FixedOpen, Modifier) => false,
            (Modifier, VariableOpen) => false,
            (Modifier, FixedOpen) => false,
            (Modifier, Modifier) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub byte: u8,
    pub chr: char,
    pub long: &'static str,
    pub typ: TokenType,
}

impl Token {
    pub const fn new(byte: u8, chr: char, long: &'static str, typ: TokenType) -> Self {
        Token {
            byte,
            chr,
            long,
            typ,
        }
    }

    pub const fn literal(byte: u8, chr: char, long: &'static str) -> Self {
        Token::new(byte, chr, long, TokenType::Literal)
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

    pub fn is_literal(&self) -> bool {
        self.typ == TokenType::Literal
    }

    pub fn is_variable(&self) -> bool {
        self.typ == TokenType::VariableOpen
    }

    pub fn is_fixed(&self) -> bool {
        self.typ == TokenType::FixedOpen
    }

    pub fn is_opener(&self) -> bool {
        self.is_fixed() || self.is_variable()
    }

    pub fn is_modifier(&self) -> bool {
        self.typ == TokenType::Modifier
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (#{})", self.chr, self.byte)
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
