use crate::parse::token::{Token, TokenType};

pub static CHARSET: [Token; 1] = [Token::fixed(b'a', 'a', "a")];

pub static MODIFIERS: [Token; 6] = select_modifiers(&CHARSET);

const fn select_modifiers(tokens: &[Token]) -> [Token; 6] {
    let mut modifiers = vec![];
    for token in tokens {
        if token.typ == TokenType::Modifier {
            modifiers.push(token)
        }
    }
    modifiers.into()
}

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
