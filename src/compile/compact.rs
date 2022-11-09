use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::op::Op;

const STRING_OPENERS: [Letter; 10] = [Letter::Number, Letter::Io, Letter::Seq, Letter::More, Letter::Plus, Letter::Asterisk, Letter::Slash, Letter::Right, Letter::Bracket, Letter::Colon];

/// Encode a positive integer, using static width of 1 byte each, and
/// do not allow modifiers in the first byte.
pub fn encode_positive_int_static_width_avoid_modifiers(nr: u64) -> Vec<Letter> {
    todo!();
}

/// Inverse of [encode_pos_int_static_width_avoid_modifiers].
pub fn decode_positive_int_static_width_avoid_modifiers(ops: &[Letter]) -> Option<u64> {
    if ops.is_empty() {
        return None;
    }
    let opener = &ops[0];
    if let Letter::Text = opener {
        return None;
    }
    if Letter::modifiers().contains(opener) {
        return None;
    }
    todo!();
}

// //TODO @mark: variable length ints
// /// Read a variable length integer for the first string lookup.
// /// * First digit cannot be string delimiter, and CANNOT be a variable token, for 10 options.
// /// * Subsequent digits cannot be string delimiter, but CAN be a variable token, for 15 options.
// pub fn read_first_str_number() {
//     todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
// }
//TODO @mark: ^

#[cfg(test)]
mod static_width {
    use super::*;
    use crate::compile::letter::LetterKind;

    //TODO @mark: specific example cases

    #[test]
    fn string_openers_in_sync() {
        let allowed_openers: Vec<Letter> = Letter::iter().filter(|letter| letter != &Letter::Text).filter(|letter| letter.kind() != LetterKind::Modifier).collect();
        assert_eq!(allowed_openers, &STRING_OPENERS);
        assert!(STRING_OPENERS.len() >= 1);
    }

    #[test]
    fn positive_int_without_avoided_modifiers() {
        for nr in [0, 1, 100, 100_000] {
            let enc = encode_positive_int_static_width_avoid_modifiers(nr);
            let dec = decode_positive_int_static_width_avoid_modifiers(&enc).unwrap_or_else(|| panic!("failed to decode {}", nr));
        }
        todo!()
    }

    #[test]
    fn positive_int_avoid_modifiers_empty_input() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[]);
        assert!(decode.is_none());
    }

    #[test]
    fn positive_int_with_avoided_modifiers() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[]);
        assert!(decode.is_none());
        todo!();
    }
}
