use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::strum_macros::EnumIter;

//TODO @mark: meaningful names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Letter {
    // Literals
    Number,
    Text,

    // Fixed
    Io,
    Seq,
    More,

    // Variable

    // Modifiers
    Hat,
    Exclamation,
    Question,
    Hash,
    Tilde,
    // Letter::literal(0, '0', "num"),
    // Letter::literal(1, '"', "str"),
    // Letter::fixed(2, 'i', "inp"),
    // Letter::fixed(3, 'n', "seq"),
    // Letter::fixed(4, '.', "more"),
    // Letter::var(5, '+', "plus"),
    // Letter::var(6, 'x', "x"),
    // Letter::var(7, '=', "eq"),
    // Letter::var(8, '>', "gt"),
    // Letter::var(9, '$', "var"),
    // Letter::var(10, ':', "forall"),
    // Letter::modi(11, '^', "hat"),
    // Letter::modi(12, '!', "exclamation"),
    // Letter::modi(13, '?', "question"),
    // Letter::modi(14, '#', "hash"),
    // Letter::modi(15, '~', "tilde"),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum LetterKind {
    /// Special letter that starts a number or text literal.
    Literal,
    /// Letter appears alone, or followed by modifiers.
    VariableOpen,
    /// Letter is followed by one other token of any type, and then optional modifiers.
    FixedOpen,
    /// Affects the previous opener, or the whole program if there is no preceding opener.
    Modifier,
}

impl Letter {
    pub fn nr(&self) -> u8 {
        match self {
            Letter::Number => 0,
            Letter::Text => 0,
            Letter::Io => 0,
            Letter::Seq => 0,
            Letter::More => 0,
            Letter::Hat => 0,
            Letter::Exclamation => 0,
            Letter::Question => 0,
            Letter::Hash => 0,
            Letter::Tilde => 0,
        }
    }

    pub fn symbol(&self) -> char {
        todo!(); //TODO @mark:
    }

    pub fn kind(&self) -> LetterKind {
        todo!(); //TODO @mark:
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use ::strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn unique_nr() {
        let unique = Letter::iter().map(|letter| letter.nr()).collect::<HashSet<_>>();
        assert_eq!(unique.len(), Letter::iter().count());
    }

    #[test]
    fn unique_symbol() {
        let unique = Letter::iter().map(|letter| letter.symbol()).collect::<HashSet<_>>();
        assert_eq!(unique.len(), Letter::iter().count());
    }

    #[test]
    fn all_kinds_present() {
        let unique = Letter::iter().map(|letter| letter.kind()).collect::<HashSet<_>>();
        for kind in LetterKind::iter() {
            assert!(unique.contains(&kind), "unused kind: {:?}", kind);
        }
    }

    #[test]
    fn letters_fit_in_half_byte() {
        let count = Letter::iter().count();
        assert!(count <= 16, "should fit in half byte (uses {})", count);
        assert!(count >= 16, "should fully utilize all bits in half byte (uses {})", count);
    }
}
