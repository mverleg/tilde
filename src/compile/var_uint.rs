use ::std::fmt;
use ::std::fmt::Formatter;
use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::compile::var_uint::DecodeError::TooLarge;
use crate::op::Op;

const STRING_OPENERS: [Letter; 10] = [Number, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon];
const STRING_FOLLOWERS: [Letter; 16] = [Number, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Hat, Exclamation, Question, Hash, Tilde, Text];
const STRING_OPENERS_VALUES: [u64; 16] = [0, u64::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX];
const STRING_FOLLOWER_VALUES: [u64; 16] = [0, 15, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

/// Encode a positive integer, using static width of 1 byte each, and
/// do not allow modifiers in the first byte.
pub fn encode_positive_int_static_width_avoid_modifiers(nr: u64) -> Vec<Letter> {
    let mut bytes = vec![];
    let opener_n = (STRING_OPENERS.len() / 2) as u64;
    if nr < opener_n {
        bytes.push(STRING_OPENERS[(nr + opener_n) as usize]);
    } else {
        bytes.push(STRING_OPENERS[(nr % opener_n) as usize]);
    }
    let middle_n = (STRING_FOLLOWERS.len() / 2) as u64;
    debug_assert!(middle_n < 16 && (middle_n as usize) < usize::MAX);
    let mut rem = nr / opener_n;
    while rem > 0 {
        rem -= 1;
        let pos = if rem < middle_n { rem + middle_n } else { rem % middle_n };
        bytes.push(STRING_FOLLOWERS[pos as usize]);
        rem = rem / middle_n;
    }
    bytes
}

/// Inverse of [encode_pos_int_static_width_avoid_modifiers].
pub fn decode_positive_int_static_width_avoid_modifiers(letters: &[Letter]) -> Result<DecodedPositiveNumber, DecodeError> {
    if letters.is_empty() {
        return Err(DecodeError::NoInput);
    }
    let opener = &letters[0];
    if let Letter::Text = opener {
        return Err(DecodeError::TextNode);
    }
    if Letter::modifiers().contains(opener) {
        return Err(DecodeError::StarsWithModifier);
    }
    let value = STRING_OPENERS_VALUES[opener.nr() as usize];
    if value >= 16 {
        return Err(DecodeError::UnexpectedNode);
    }
    let open_n = (STRING_OPENERS.len() / 2) as u64;
    if value >= open_n {
        return Ok(DecodedPositiveNumber { end_index: 0, number: value - open_n });
    };
    let mut nr = value;
    let mut multiplier = open_n;
    let follow_n = (STRING_FOLLOWERS.len() / 2) as u64;
    let follower_letters = letters
        .iter()
        .enumerate()
        .skip(1);
    for (i, letter) in follower_letters {
        let mut value = STRING_FOLLOWER_VALUES[letter.nr() as usize];
        if value >= 16 {
            return Err(DecodeError::UnexpectedNode);
        }
        value += 1;
        if value > follow_n {
            let scale = multiplier
                .checked_mul(value - follow_n)
                .ok_or(DecodeError::TooLarge)?;
            nr = nr
                .checked_add(scale)
                .ok_or(DecodeError::TooLarge)?;
            return Ok(DecodedPositiveNumber { end_index: i, number: nr });
        }
        let scale = multiplier
            .checked_mul(value)
            .ok_or(DecodeError::TooLarge)?;
        nr = nr
            .checked_add(scale)
            .ok_or(DecodeError::TooLarge)?;
        multiplier = multiplier
            .checked_mul(follow_n)
            .ok_or(DecodeError::TooLarge)?;
    }
    Err(DecodeError::NoEndMarker)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecodedPositiveNumber {
    pub end_index: usize,
    pub number: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecodeError {
    NoInput,
    TextNode,
    UnexpectedNode,
    StarsWithModifier,
    TooLarge,
    NoEndMarker,
}

impl fmt::Display for DecodeError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DecodeError::NoInput => "number to decode is empty",
                DecodeError::TextNode => "encountered unexpected text node while decoding number",
                DecodeError::UnexpectedNode => "encountered unexpected (non-text) node while decoding number",
                DecodeError::StarsWithModifier => "number to decode starts with a modifier, which is not allowed here",
                DecodeError::TooLarge => "number to decode is too large",
                DecodeError::NoEndMarker => "unexpected end while decoding number; last letter should be marked",
            }
        )
    }
}

#[cfg(test)]
mod constants_in_sync {
    use super::*;
    use crate::compile::letter::LetterKind;

    fn select_letters(predicate: impl Fn(&Letter) -> bool) -> Vec<Letter> {
        let mut allowed: Vec<Letter> = Letter::iter()
            .filter(|letter| !Letter::modifiers().contains(letter))
            .filter(|letter| letter != &Text)
            .filter(&predicate)
            .collect();
        // Put Text and modifiers in the second half (if allowed) to keep the order more stable.
        for letter in Letter::modifiers()
            .into_iter()
            .chain(Some(Text).into_iter())
        {
            if predicate(&letter) {
                allowed.push(letter);
            }
        }
        if allowed.len() % 2 != 0 {
            allowed.pop();
        }
        assert_eq!(allowed.len() % 2, 0, "should be an even number of letters, because any odd tail will be ignored");
        assert!(allowed.len() >= 2, "must allow at least two letters, since half are used to indicate closing");
        allowed
    }

    fn reverse_letter_values(letters: &[Letter]) -> Vec<u64> {
        let mut expected = vec![u64::MAX; 16];
        for (value, letter) in letters.iter().enumerate() {
            expected[letter.nr() as usize] = value.try_into().unwrap();
        }
        assert_eq!(
            expected
                .iter()
                .filter(|val| **val < 16)
                .count(),
            letters.len()
        );
        expected
    }

    /// The first letter cannot start with a modifier, because those can be used to modify
    /// the start of the string itself, thus not being part of the (first) letter.
    /// It can also not contain a Text letter, because that may signal the end of the
    /// series of numbers (i.e. after the last number, not to be confused with the next number).
    #[test]
    fn openers() {
        assert_eq!(STRING_OPENERS, select_letters(|letter| letter.kind() != LetterKind::Modifier).as_slice());
    }

    /// After the start of the number, everything is allowed - encoutnering any of the second
    /// half of letters will signal the end of the number.
    #[test]
    fn followers() {
        assert_eq!(STRING_FOLLOWERS, select_letters(|_letter| true).as_slice());
    }

    #[test]
    fn reverse_openers() {
        let expected = reverse_letter_values(&STRING_OPENERS);
        assert_eq!(STRING_OPENERS_VALUES, expected.as_slice());
    }

    #[test]
    fn reverse_followers() {
        let expected = reverse_letter_values(&STRING_FOLLOWERS);
        assert_eq!(STRING_FOLLOWER_VALUES, expected.as_slice());
    }
}

#[cfg(test)]
mod static_width {
    use std::collections::HashSet;

    use super::*;
    use crate::compile::letter::LetterKind;

    #[test]
    fn positive_int_avoided_modifiers_encoding_examples() {
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(0), vec![Asterisk]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(4), vec![Colon]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(5), vec![Number, Right]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(9), vec![Plus, Right]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(10), vec![Number, Bracket]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(39), vec![Plus, Hash]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(40), vec![Number, Number, Right]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(45), vec![Number, Io, Right]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(69), vec![Plus, Asterisk, Right]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(70), vec![Number, Slash, Right]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(1999), vec![Plus, Slash, Slash, Hash]);
        assert_eq!(encode_positive_int_static_width_avoid_modifiers(2000), vec![Number, Number, Number, Number, Right]);
    }

    #[test]
    fn positive_int_avoided_modifiers_decoding_examples() {
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Asterisk]).unwrap(), DecodedPositiveNumber { end_index: 0, number: 0 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Colon]).unwrap(), DecodedPositiveNumber { end_index: 0, number: 4 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Number, Right]).unwrap(), DecodedPositiveNumber { end_index: 1, number: 5 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Plus, Right, Io]).unwrap(), DecodedPositiveNumber { end_index: 1, number: 9 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Number, Bracket]).unwrap(), DecodedPositiveNumber { end_index: 1, number: 10 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Plus, Hash, Io]).unwrap(), DecodedPositiveNumber { end_index: 1, number: 39 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Number, Number, Right, Io]).unwrap(), DecodedPositiveNumber { end_index: 2, number: 40 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Plus, Asterisk, Right]).unwrap(), DecodedPositiveNumber { end_index: 2, number: 69 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Number, Slash, Right]).unwrap(), DecodedPositiveNumber { end_index: 2, number: 70 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Number, Io, Right]).unwrap(), DecodedPositiveNumber { end_index: 2, number: 45 });
        assert_eq!(
            decode_positive_int_static_width_avoid_modifiers(&[Plus, Slash, Slash, Hash]).unwrap(),
            DecodedPositiveNumber { end_index: 3, number: 1999 }
        );
        assert_eq!(
            decode_positive_int_static_width_avoid_modifiers(&[Number, Number, Number, Number, Right]).unwrap(),
            DecodedPositiveNumber { end_index: 4, number: 2000 }
        );
    }

    #[test]
    fn positive_int_without_avoided_modifiers() {
        let nrs = (0..100).chain((0..10).map(|n| n * 2 - n));
        for nr in nrs {
            let enc = encode_positive_int_static_width_avoid_modifiers(nr);
            let dec = decode_positive_int_static_width_avoid_modifiers(&enc).unwrap_or_else(|_| panic!("failed to decode {}", nr));
            assert_eq!(nr, dec.number);
        }
    }

    #[test]
    fn positive_int_avoid_modifiers_empty_input() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[]);
        assert_eq!(decode, Err(DecodeError::NoInput));
    }

    #[test]
    fn positive_int_starting_with_modifiers() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Letter::modifiers()[0], Asterisk]);
        assert_eq!(decode, Err(DecodeError::StarsWithModifier));
    }

    #[test]
    fn positive_int_non_terminated_number() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io]);
        assert_eq!(decode, Err(DecodeError::NoEndMarker));
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io, Io, Io]);
        assert_eq!(decode, Err(DecodeError::NoEndMarker));
    }

    #[test]
    fn positive_int_decode_with_text_opener() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Text, Hash]);
        assert_eq!(decode, Err(DecodeError::TextNode));
    }

    #[test]
    /// Because the number of letters is even, and Text is not allowed at the end, (at least) one other letter
    /// should also not be allowed. This checks that that is handled gracefully.
    fn positive_int_decode_with_unused_letter() {
        for list in [STRING_OPENERS.as_slice(), STRING_FOLLOWERS.as_slice()] {
            let mut unused = HashSet::new();
            for letter in Letter::iter() {
                unused.insert(letter);
            }
            unused.remove(&Text);
            for letter in Letter::modifiers() {
                unused.remove(&letter);
            }
            for letter in list {
                unused.remove(&letter);
            }
            if unused.is_empty() {
                eprintln!("no unused letters")
            }
            for letter in unused {
                let dec = decode_positive_int_static_width_avoid_modifiers(&[Number, letter, Hash]);
                assert_eq!(dec, Err(DecodeError::UnexpectedNode), "list len: {}, letter: {}", list.len(), letter.symbol());
            }
        }
    }

    #[test]
    fn positive_int_overflow_in_decode() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io; 100]);
        assert_eq!(decode, Err(DecodeError::TooLarge));
    }
}
