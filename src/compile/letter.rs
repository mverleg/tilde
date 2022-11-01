use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;

#[derive(Debug, Clone, Copy)]
//TODO @mark: remove?
pub enum LetterType {
    /// Special letter that starts a number or text literal.
    Literal,
    /// Letter appears alone, or followed by modifiers.
    VariableOpen,
    /// Letter is followed by one other token of any type, and then optional modifiers.
    FixedOpen,
    /// Affects the previous opener, or the whole program if there is no preceding opener.
    Modifier,
}

impl PartialEq for LetterType {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        use LetterType::*;
        match (self, other) {
            (VariableOpen, VariableOpen) => true,
            (VariableOpen, _) => false,
            (FixedOpen, FixedOpen) => true,
            (FixedOpen, _) => false,
            (Modifier, Modifier) => true,
            (Modifier, _) => false,
            (Literal, Literal) => true,
            (Literal, _) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Letter {
    pub byte: u8,
    pub chr: char,
    pub long: &'static str,
    pub typ: LetterType,
}

impl Letter {
    pub const fn new(
        byte: u8,
        chr: char,
        long: &'static str,
        typ: LetterType,
    ) -> Self {
        Letter { byte, chr, long, typ }
    }

    pub const fn literal(
        byte: u8,
        chr: char,
        long: &'static str,
    ) -> Self {
        Letter::new(byte, chr, long, LetterType::Literal)
    }

    pub const fn var(
        byte: u8,
        chr: char,
        long: &'static str,
    ) -> Self {
        Letter::new(byte, chr, long, LetterType::VariableOpen)
    }

    pub const fn fixed(
        byte: u8,
        chr: char,
        long: &'static str,
    ) -> Self {
        Letter::new(byte, chr, long, LetterType::FixedOpen)
    }

    pub const fn modi(
        byte: u8,
        chr: char,
        long: &'static str,
    ) -> Self {
        Letter::new(byte, chr, long, LetterType::Modifier)
    }

    pub fn is_literal(&self) -> bool {
        self.typ == LetterType::Literal
    }

    pub fn is_variable(&self) -> bool {
        self.typ == LetterType::VariableOpen
    }

    pub fn is_fixed(&self) -> bool {
        self.typ == LetterType::FixedOpen
    }

    pub fn is_opener(&self) -> bool {
        self.is_fixed() || self.is_variable()
    }

    pub fn is_modifier(&self) -> bool {
        self.typ == LetterType::Modifier
    }
}

impl fmt::Display for Letter {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{} (#{})", self.chr, self.byte)
    }
}

impl PartialEq for Letter {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.byte == other.byte
    }
}

impl Eq for Letter {}

impl Hash for Letter {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        self.byte.hash(state)
    }
}
