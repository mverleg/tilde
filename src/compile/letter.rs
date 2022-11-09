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
    Plus,
    Asterisk,
    Slash,
    Right,
    Bracket,
    Colon,

    // Modifiers
    Hat,
    Exclamation,
    Question,
    Hash,
    Tilde,
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
        use self::Letter::*;
        match self {
            Number => 0,
            Text => 1,

            Io => 2,
            Seq => 3,
            More => 4,

            Plus => 5,
            Asterisk => 6,
            Slash => 7,
            Right => 8,
            Bracket => 9,
            Colon => 10,

            Hat => 11,
            Exclamation => 12,
            Question => 13,
            Hash => 14,
            Tilde => 15,
        }
    }

    pub fn symbol(&self) -> char {
        //TODO @mark: reconsider (except 0 and ")
        use self::Letter::*;
        match self {
            Number => '0',
            Text => '"',

            Io => '$',
            Seq => '%',
            More => '&',

            Plus => '+',
            Asterisk => '*',
            Slash => '/',
            Right => '>',
            Bracket => '[',
            Colon => ':',

            Hat => '^',
            Exclamation => '!',
            Question => '?',
            Hash => '#',
            Tilde => '~',
        }
    }

    pub fn kind(&self) -> LetterKind {
        use self::Letter::*;
        use self::LetterKind::*;
        match self {
            Number => Literal,
            Text => Literal,

            Io => FixedOpen,
            Seq => FixedOpen,
            More => FixedOpen,

            Plus => VariableOpen,
            Asterisk => VariableOpen,
            Slash => VariableOpen,
            Right => VariableOpen,
            Bracket => VariableOpen,
            Colon => VariableOpen,

            Hat => Modifier,
            Exclamation => Modifier,
            Question => Modifier,
            Hash => Modifier,
            Tilde => Modifier,
        }
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
