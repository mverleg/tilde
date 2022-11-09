use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::op::Op;

const STRING_OPENERS: [Letter; 10] = [Number, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon];
const STRING_FOLLOWERS: [Letter; 14] = [Number, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Hat, Exclamation, Question, Hash];

/// Encode a positive integer, using static width of 1 byte each, and
/// do not allow modifiers in the first byte.
pub fn encode_positive_int_static_width_avoid_modifiers(nr: u64) -> Vec<Letter> {
    let mut bytes = vec![];
    let opener_n = (STRING_OPENERS.len() / 2) as u64;
    if nr < opener_n {
        return vec![STRING_OPENERS[(nr + opener_n) as usize]];
    }
    let follow_n = (STRING_FOLLOWERS.len() / 2) as u64;
    bytes.push(STRING_FOLLOWERS[((nr % follow_n) + follow_n) as usize]);
    let mut rem = nr / follow_n;
    debug_assert!(follow_n < 16 && (follow_n as usize) < usize::MAX);
    while rem >= opener_n {
        let value = rem % follow_n;
        bytes.push(STRING_FOLLOWERS[value as usize]);
        rem /= follow_n;
    }
    debug_assert!(follow_n < 16 && (follow_n as usize) < usize::MAX);
    bytes.push(STRING_OPENERS[rem as usize]);
    bytes.reverse();
    bytes
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
        let mut allowed_followers: Vec<Letter> = Letter::iter().filter(|letter| letter != &Letter::Text).collect();
        if allowed_followers.len() % 2 != 0 {
            allowed_followers.pop();
        }
        let mut allowed_openers: Vec<Letter> = allowed_followers.iter().cloned().filter(|letter| letter.kind() != LetterKind::Modifier).collect();
        if allowed_openers.len() % 2 != 0 {
            allowed_openers.pop();
        }
        assert_eq!(allowed_openers, &STRING_OPENERS);
        assert_eq!(allowed_followers, &STRING_FOLLOWERS);
        assert!(STRING_OPENERS.len() >= 1);
    }

    #[test]
    fn positive_int_without_avoided_modifiers() {
        for nr in [0, 1, 4, 5, 9, 10, 100, 100_000] {
            let enc = encode_positive_int_static_width_avoid_modifiers(nr);
            println!("{} => [{}]", nr, enc.iter().map(|letter| letter.symbol().to_string()).collect::<Vec<_>>().join(", "));
            //TODO @mark: TEMPORARY! REMOVE THIS!
            //let dec = decode_positive_int_static_width_avoid_modifiers(&enc).unwrap_or_else(|| panic!("failed to decode {}", nr));
            //TODO @mark: ^
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
