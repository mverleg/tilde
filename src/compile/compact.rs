use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::op::Op;

//TODO @mark: any non-first number can start with openers, so make separate versions

const STRING_OPENERS: [Letter; 10] = [Number, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon];
const STRING_SINGLE: [Letter; 10] = STRING_OPENERS;
const STRING_MIDDLE: [Letter; 16] = [Number, Text, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Hat, Exclamation, Question, Hash, Tilde];
const STRING_CLOSER: [Letter; 14] = [Number, Io, Seq, More, Plus, Asterisk, Slash, Right, Bracket, Colon, Hat, Exclamation, Question, Hash];

/// Encode a positive integer, using static width of 1 byte each, and
/// do not allow modifiers in the first byte.
pub fn encode_positive_int_static_width_avoid_modifiers(nr: u64) -> Vec<Letter> {
    let mut bytes = vec![];
    let opener_n = (STRING_OPENERS.len() / 2) as u64;
    if nr < opener_n {
        bytes.push(STRING_OPENERS[(nr + opener_n) as usize]);
        eprintln!("{nr}: ONLY = {}", nr + opener_n); //TODO @mark: TEMPORARY! REMOVE THIS!
    } else {
        bytes.push(STRING_OPENERS[(nr % opener_n) as usize]);
        eprintln!("{nr}: FIRST = {}", nr % opener_n); //TODO @mark: TEMPORARY! REMOVE THIS!
    }
    let middle_n = (STRING_MIDDLE.len() / 2) as u64;
    debug_assert!(middle_n < 16 && (middle_n as usize) < usize::MAX);
    let mut rem = nr / opener_n;
    eprintln!("{nr}: rem_i = {rem}"); //TODO @mark: TEMPORARY! REMOVE THIS!
    while rem > 0 {
        let pos = if rem < middle_n { rem + middle_n } else { rem % middle_n };
        bytes.push(STRING_MIDDLE[pos as usize]);
        rem = rem / middle_n;
        eprintln!("{nr}: val = {pos}, rem = {rem}"); //TODO @mark: TEMPORARY! REMOVE THIS!
    }
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

#[cfg(test)]
mod constants_in_sync {
    use super::*;
    use crate::compile::letter::LetterKind;

    fn select_letters(predicate: impl Fn(&Letter) -> bool) -> Vec<Letter> {
        let mut allowed: Vec<Letter> = Letter::iter()
            .filter(predicate)
            .collect();
        if allowed.len() % 2 != 0 {
            allowed = allowed
                .into_iter()
                .filter(|letter| letter != &Text)
                .collect();
            if allowed.len() % 2 != 0 {
                allowed.pop();
            }
        }
        assert_eq!(allowed.len() % 2, 0, "should be an even number of letters, because any odd tail will be ignored");
        assert!(allowed.len() >= 2, "must allow at least two letters, since half are used to indicate closing");
        allowed
    }

    /// The first text number cannot start with a modifier, because those can be used to modify
    /// the text opener itself, thus not being part of the (first) number.
    #[test]
    fn openers() {
        assert_eq!(STRING_OPENERS, select_letters(|letter| letter.kind() != LetterKind::Modifier).as_slice());
    }

    /// In the middle of numbers, all letters are allowed.
    #[test]
    fn middles() {
        assert_eq!(STRING_MIDDLE, select_letters(|_letter| true).as_slice());
    }

    /// A text number cannot end with a text token, as that would cause ambiguity between
    /// continuing the number or closing the whole text.
    #[test]
    fn closers() {
        assert_eq!(STRING_CLOSER, select_letters(|letter| letter != &Text).as_slice());
    }

    /// If there is a single byte (opener == closer), exclude Text and modifiers
    #[test]
    fn singles() {
        assert_eq!(STRING_SINGLE, select_letters(|letter| letter != &Text && letter.kind() != LetterKind::Modifier).as_slice());
    }
}

#[cfg(test)]
mod static_width {
    use super::*;
    use crate::compile::letter::LetterKind;

    #[test]
    fn positive_int_avoided_modifiers__encoding_examples() {
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
    fn positive_int_without_avoided_modifiers() {
        for nr in [0, 1, 4, 5, 9, 10, 100, 100_000] {
            let enc = encode_positive_int_static_width_avoid_modifiers(nr);
            println!(
                "{} => [{}]",
                nr,
                enc.iter()
                    .map(|letter| letter.symbol().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
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
