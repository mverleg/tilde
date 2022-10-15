use crate::compile::Letter;

pub static ALPHABET: [Letter; 16] = [
    Letter::literal(0, '0', "num"),
    Letter::literal(1, '"', "str"),
    Letter::fixed(2, 'i', "inp"),
    Letter::fixed(3, 'n', "seq"),
    Letter::fixed(4, '.', "more"),
    Letter::var(5, '+', "plus"),
    Letter::var(6, 'x', "x"),
    Letter::var(7, '=', "eq"),
    Letter::var(8, '>', "gt"),
    Letter::var(9, '$', "var"),
    Letter::var(10, ':', "forall"),
    Letter::modi(11, '^', "hat"),
    Letter::modi(12, '!', "exclamation"),
    Letter::modi(13, '?', "question"),
    Letter::modi(14, '#', "hash"),
    Letter::modi(15, '~', "tilde"),
];
//TODO @mverleg: close block token

#[cfg(test)]
mod tests {
    use ::std::collections::HashSet;
    use ::std::hash::Hash;

    use super::*;

    fn check_prop_unique<T: Eq + Hash>(getter: fn(&Letter) -> T) {
        let mut seen = HashSet::new();
        for c in &ALPHABET {
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
