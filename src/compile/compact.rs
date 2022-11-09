use ::strum::IntoEnumIterator;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecodedPositiveNumber {
    pub end_index: usize,
    pub number: u64,
}

/// Inverse of [encode_pos_int_static_width_avoid_modifiers].
pub fn decode_positive_int_static_width_avoid_modifiers(ops: &[Letter]) -> Result<DecodedPositiveNumber, &'static str> {
    if ops.is_empty() {
        return Err("number to decode is empty");
    }
    let opener = &ops[0];
    if let Letter::Text = opener {
        return Err("number contains text node as opener");
    }
    if Letter::modifiers().contains(opener) {
        return Err("number starts with opener");
    }
    let value = STRING_OPENERS_REV[opener.nr() as usize];
    debug_assert!(value < 16);
    let open_n = (STRING_OPENERS.len() / 2) as u64;
    if value >= open_n {
        return Ok(DecodedPositiveNumber { end_index: 0, number: value - open_n });
    };
    let mut nr = value;
    let mut multiplier = 1u64;
    let follow_n = (STRING_FOLLOWERS.len() / 2) as u64;
    for i in 1..ops.len() {
        if let Letter::Text = opener {
            return Err("number contains text node as non-opener");
        }
        multiplier = multiplier
            .checked_mul(follow_n)
            .ok_or_else(|| "number is too large (in: multiplier)")?;
        let value = STRING_OPENERS_REV[opener.nr() as usize];
        debug_assert!(value < 16);
        if value >= follow_n {
            let scale = multiplier
                .checked_mul(value - follow_n)
                .ok_or_else(|| "number is too large (in: final scale)")?;
            nr = nr
                .checked_add(scale)
                .ok_or_else(|| "number is too large (in: final sum)")?;
            return Ok(DecodedPositiveNumber { end_index: i, number: nr });
        }
        let scale = multiplier
            .checked_mul(value)
            .ok_or_else(|| "number is too large (in: scale)")?;
        nr = nr
            .checked_add(scale)
            .ok_or_else(|| "number is too large (in: sum)")?;
    }
    Err("unexpected end of number while decoding (last letter should be marked)")
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
    }

    #[test]
    fn positive_int_avoided_modifiers_decoding_examples() {
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Asterisk]).unwrap(), DecodedPositiveNumber { end_index: 0, number: 0 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Colon]).unwrap(), DecodedPositiveNumber { end_index: 0, number: 4 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Number, Right]).unwrap(), DecodedPositiveNumber { end_index: 1, number: 5 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Plus, Right]).unwrap(), DecodedPositiveNumber { end_index: 1, number: 9 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Number, Bracket]).unwrap(), DecodedPositiveNumber { end_index: 1, number: 10 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Plus, Hash]).unwrap(), DecodedPositiveNumber { end_index: 1, number: 39 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Number, Number, Right]).unwrap(), DecodedPositiveNumber { end_index: 2, number: 40 });
        assert_eq!(decode_positive_int_static_width_avoid_modifiers(&[Number, Io, Right]).unwrap(), DecodedPositiveNumber { end_index: 2, number: 45 });
    }

    #[test]
    fn positive_int_without_avoided_modifiers() {
        for nr in 0..=1000 {
            let enc = encode_positive_int_static_width_avoid_modifiers(nr);
            println!(
                "{} => [{}]",
                nr,
                enc.iter()
                    .map(|letter| letter.symbol().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let dec = decode_positive_int_static_width_avoid_modifiers(&enc).unwrap_or_else(|_| panic!("failed to decode {}", nr));
            assert_eq!(nr, dec.number);
        }
    }

    #[test]
    fn positive_int_avoid_modifiers_empty_input() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[]);
        assert!(decode.is_err());
    }

    #[test]
    fn positive_int_starting_with_modifiers() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Letter::modifiers()[0], Asterisk]);
        assert!(decode.is_err());
    }

    #[test]
    fn positive_int_non_terminated_number() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io, Io, Io]);
        assert!(decode.is_err());
    }

    #[test]
    fn positive_int_overflow_in_decode() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[Io; 100]);
        assert!(decode.is_err());
    }
}
