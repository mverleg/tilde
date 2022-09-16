
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Token appears alone, or followed by modifiers.
    VariableOpen,
    /// Token is followed by one other token of any type, and then optional modifiers.
    FixedOpen,
    /// Affects the previous opener, or the whole program if there is no preceding opener.
    Modifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    byte: u8,
    chr: char,
    long: String,
}

impl Token {
    pub const fn new(byte: u8, chr: char, long: impl Into<String>) -> Self {
        Token {
            byte,
            chr,
            long,
        }
    }
}

const CHARSET: [Token; u8::MAX as usize] = [
    Token::new(b'a', 'a', "a"),
];
