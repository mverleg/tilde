use crate::parse::Token;

pub static TOKENSET: [Token; 100] = [
    Token::fixed(b' ', ' ', "space"),
    Token::fixed(b'!', '!', "!"),
    Token::fixed(b'"', '"', "\""),
    Token::fixed(b'#', '#', "#"),
    Token::fixed(b'$', '$', "$"),
    Token::fixed(b'%', '%', "%"),
    Token::fixed(b'&', '&', "&"),
    Token::fixed(b'\'', '\'', "'"),
    Token::fixed(b'(', '(', "("),
    Token::fixed(b')', ')', ")"),
    Token::fixed(b'*', '*', "*"),
    Token::fixed(b'+', '+', "+"),
    Token::fixed(b',', ',', ","),
    Token::fixed(b'-', '-', "-"),
    Token::fixed(b'.', '.', "."),
    Token::fixed(b'/', '/', "/"),
    Token::fixed(b'0', '0', "0"),
    Token::fixed(b'1', '1', "1"),
    Token::fixed(b'2', '2', "2"),
    Token::fixed(b'3', '3', "3"),
    Token::fixed(b'4', '4', "4"),
    Token::fixed(b'5', '5', "5"),
    Token::fixed(b'6', '6', "6"),
    Token::fixed(b'7', '7', "7"),
    Token::fixed(b'8', '8', "8"),
    Token::fixed(b'9', '9', "9"),
    Token::fixed(b':', ':', ":"),
    Token::fixed(b';', ';', ";"),
    Token::fixed(b'<', '<', "<"),
    Token::fixed(b'=', '=', "="),
    Token::fixed(b'>', '>', ">"),
    Token::fixed(b'?', '?', "?"),
    Token::fixed(b'@', '@', "@"),
    Token::fixed(b'A', 'A', "A"),
    Token::fixed(b'B', 'B', "B"),
    Token::fixed(b'C', 'C', "C"),
    Token::fixed(b'D', 'D', "D"),
    Token::fixed(b'E', 'E', "E"),
    Token::fixed(b'F', 'F', "F"),
    Token::fixed(b'G', 'G', "G"),
    Token::fixed(b'H', 'H', "H"),
    Token::fixed(b'I', 'I', "I"),
    Token::fixed(b'J', 'J', "J"),
    Token::fixed(b'K', 'K', "K"),
    Token::fixed(b'L', 'L', "L"),
    Token::fixed(b'M', 'M', "M"),
    Token::fixed(b'N', 'N', "N"),
    Token::fixed(b'O', 'O', "O"),
    Token::fixed(b'P', 'P', "P"),
    Token::fixed(b'Q', 'Q', "Q"),
    Token::fixed(b'R', 'R', "R"),
    Token::fixed(b'S', 'S', "S"),
    Token::fixed(b'T', 'T', "T"),
    Token::fixed(b'U', 'U', "U"),
    Token::fixed(b'V', 'V', "V"),
    Token::fixed(b'W', 'W', "W"),
    Token::fixed(b'X', 'X', "X"),
    Token::fixed(b'Y', 'Y', "Y"),
    Token::fixed(b'Z', 'Z', "Z"),
    Token::fixed(b'[', '[', "["),
    Token::fixed(b'\\', '\\', "\\"),
    Token::fixed(b']', ']', "]"),
    Token::fixed(b'^', '^', "^"),
    Token::fixed(b'_', '_', "_"),
    Token::fixed(b'`', '`', "`"),
    Token::fixed(b'a', 'a', "a"),
    Token::fixed(b'b', 'b', "b"),
    Token::fixed(b'c', 'c', "c"),
    Token::fixed(b'd', 'd', "d"),
    Token::fixed(b'e', 'e', "e"),
    Token::fixed(b'f', 'f', "f"),
    Token::fixed(b'g', 'g', "g"),
    Token::fixed(b'h', 'h', "h"),
    Token::fixed(b'i', 'i', "i"),
    Token::fixed(b'j', 'j', "j"),
    Token::fixed(b'k', 'k', "k"),
    Token::fixed(b'l', 'l', "l"),
    Token::fixed(b'm', 'm', "m"),
    Token::fixed(b'n', 'n', "n"),
    Token::fixed(b'o', 'o', "o"),
    Token::fixed(b'p', 'p', "p"),
    Token::fixed(b'q', 'q', "q"),
    Token::fixed(b'r', 'r', "r"),
    Token::fixed(b's', 's', "s"),
    Token::fixed(b't', 't', "t"),
    Token::fixed(b'u', 'u', "u"),
    Token::fixed(b'v', 'v', "v"),
    Token::fixed(b'w', 'w', "w"),
    Token::fixed(b'x', 'x', "x"),
    Token::fixed(b'y', 'y', "y"),
    Token::fixed(b'z', 'z', "z"),
    Token::fixed(b'{', '{', "{"),
    Token::fixed(b'|', '|', "|"),
    Token::fixed(b'}', '}', "}"),
    Token::fixed(b'~', '~', "~"),
    Token::fixed(b')', ')', ")"),
    Token::modi(252, '@', "harder"),
    Token::modi(253, '#', "faster"),
    Token::modi(254, '$', "stronger"),
    Token::modi(255, '%', "larger"),
];

#[cfg(test)]
mod tests {
    use ::std::collections::HashSet;
    use ::std::hash::Hash;

    use super::*;

    fn check_prop_unique<T: Eq + Hash>(getter: fn(&Token) -> T) {
        let mut seen = HashSet::new();
        for c in &TOKENSET {
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
