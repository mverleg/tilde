use crate::parse::Token;

pub static TOKENSET: [Token; 16] = [
    Token::fixed(0, '0', "num"),
    Token::fixed(1, 'i', "inp"),
    Token::fixed(2, 'n', "seq"),
    Token::fixed(3, '.', "more"),
    Token::var(4, '"', "str"),
    Token::var(5, '+', "plus"),
    Token::var(6, 'x', "x"),
    Token::var(7, '=', "eq"),
    Token::var(8, '>', "gt"),
    Token::var(9, '$', "var"),
    Token::var(10, ':', "forall"),
    Token::modi(11, '^', "hat"),
    Token::modi(12, '!', "exclamation"),
    Token::modi(13, '?', "question"),
    Token::modi(14, '#', "hash"),
    Token::modi(15, '~', "tilde"),
];
//TODO @mverleg: close block token

#[cfg(test)]
mod tests {
    use ::std::collections::HashSet;
    use ::std::hash::Hash;

    use super::*;

    fn check_prop_unique<T: Eq + Hash>(getter: fn(&Token) -> T) {
        let mut seen = HashSet::new();
        for c in &TOKENSET {
            if !seen.insert(getter(c)) {
                panic!("duplicate char: {c}")
            }
        }
        assert_eq!(seen.len(), 16)
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
