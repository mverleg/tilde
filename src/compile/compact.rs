use ::std::fmt;
use ::std::fmt::Formatter;
use ::strum::IntoEnumIterator;

use crate::compile::compact::DecodeError::TooLarge;
use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::op::Op;

//TODO @mark: any non-first number can start with openers, so make separate versions

const STRING_OPENERS: [Letter; 10] = [Number, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon];
const STRING_FOLLOWERS: [Letter; 14] = [Number, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Hat, Exclamation, Question, Hash];
const STRING_OPENERS_REV: [u64; 16] = [0, u64::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX];
const STRING_FOLLOWERS_REV: [u64; 16] = [0, u64::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, u64::MAX];

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
    let value = STRING_OPENERS_REV[opener.nr() as usize];
    debug_assert!(value < 16, "wrong value for opener {} (index {})", opener.symbol(), opener.nr());
    let open_n = (STRING_OPENERS.len() / 2) as u64;
    if value >= open_n {
        return Ok(DecodedPositiveNumber { end_index: 0, number: value - open_n });
    };
    eprintln!("{} `{}` (first value)", value, opener.symbol()); //TODO @mark: TEMPORARY! REMOVE THIS!
    let mut nr = value;
    let mut multiplier = open_n;
    let follow_n = (STRING_FOLLOWERS.len() / 2) as u64;
    for (i, letter) in letters
        .iter()
        .enumerate()
        .skip(1)
    {
        if let Letter::Text = letter {
            return Err(DecodeError::TextNode);
        }
        let value = STRING_FOLLOWERS_REV[letter.nr() as usize] + 1;
        debug_assert!(value < 16, "wrong value for non-opener {} (index {})", letter.symbol(), letter.nr());
        if value >= follow_n {
            eprintln!("{} + {} * {} `{}` (last)", nr, multiplier, value - follow_n, letter.symbol()); //TODO @mark: TEMPORARY! REMOVE THIS!
            let scale = multiplier
                .checked_mul(value - follow_n)
                .ok_or(DecodeError::TooLarge)?;
            nr = nr
                .checked_add(scale)
                .ok_or(DecodeError::TooLarge)?;
            return Ok(DecodedPositiveNumber { end_index: i, number: nr });
        }
        eprintln!("{} + {} * {} `{}`", nr, multiplier, value, letter.symbol()); //TODO @mark: TEMPORARY! REMOVE THIS!
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
            .filter(|letter| letter != &Text)
            .filter(predicate)
            .collect();
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

    /// The first text number cannot start with a modifier, because those can be used to modify
    /// the text opener itself, thus not being part of the (first) number. Text token is also disallowed.
    #[test]
    fn openers() {
        assert_eq!(STRING_OPENERS, select_letters(|letter| letter.kind() != LetterKind::Modifier).as_slice());
    }

    /// In the middle of numbers, text token is not allowed, but modifiers are okay.
    #[test]
    fn followers() {
        assert_eq!(STRING_FOLLOWERS, select_letters(|_letter| true).as_slice());
    }

    #[test]
    fn reverse_openers() {
        let expected = reverse_letter_values(&STRING_OPENERS);
        assert_eq!(STRING_OPENERS_REV, expected.as_slice());
    }

    #[test]
    fn reverse_followers() {
        let expected = reverse_letter_values(&STRING_FOLLOWERS);
        assert_eq!(STRING_FOLLOWERS_REV, expected.as_slice());
    }
}

#[cfg(test)]
mod static_width {
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
    fn tmp_for_log() {
        //TODO @mark: TEMPORARY! REMOVE THIS!
        for nr in 0..=2010 {
            let enc = encode_positive_int_static_width_avoid_modifiers(nr);
            println!(
                "{}:  {}  ===  {}",
                nr,
                enc.iter()
                    .map(|l| STRING_FOLLOWERS_REV[l.nr() as usize].to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                enc.iter()
                    .map(|l| l.symbol().to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            //TODO @mark: TEMPORARY! REMOVE THIS!
        }
        panic!()
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
    fn positive_int_decode_with_text_letter() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Text, Hash]);
        assert_eq!(decode, Err(DecodeError::TextNode));
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io, Text, Hash]);
        assert_eq!(decode, Err(DecodeError::TextNode));
    }

    #[test]
    fn positive_int_overflow_in_decode() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io; 100]);
        assert_eq!(decode, Err(DecodeError::TooLarge));
    }
}
