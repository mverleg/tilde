use ::std::fmt;
use ::std::fmt::Formatter;
use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::compile::parse::Pos;
use crate::compile::var_uint::decode_uint_no_modifier_at_start;
use crate::compile::var_uint::encode_uint_allow_modifiers;
use crate::compile::var_uint::encode_uint_no_modifier_at_start;
use crate::compile::var_uint::DecodeError;
use crate::compile::var_uint::DecodeError::TooLarge;
use crate::op::Op;
use crate::tilde_log;
use crate::Value::Num;
use crate::NR;
use crate::UINT;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Closer {
    Text,
    Number,
}

///
/// Note: the first letter cannot be a modifier or number token, so if the length is zero
/// and the closer is supposed to be number, we have too pick text token instead.
pub fn encode_uint_vec(
    nrs: &[UINT],
    closer: Closer,
) -> Vec<Letter> {
    if nrs.is_empty() {
        return vec![Text];
    }
    let mut letters = vec![];
    letters.extend(encode_uint_no_modifier_at_start(nrs[0]));
    for nr in nrs.iter().skip(1) {
        letters.extend(encode_uint_allow_modifiers(*nr));
    }
    letters.push(match closer {
        Closer::Text => Text,
        Closer::Number => Number,
    });
    letters
}

pub fn decode_uint_vec(letters: &[Letter]) -> Result<(Pos<Vec<UINT>>, Closer), DecodeError> {
    let mut is_first = true;
    let mut pos = 0;
    let mut nrs = vec![];
    loop {
        if pos >= letters.len() {
            tilde_log!("uint_vec without end marker, interpreting as text");
            return Ok((Pos { value: nrs, end_index: pos }, Closer::Text));
        }
        if letters[pos] == Text {
            return Ok((Pos { value: nrs, end_index: pos }, Closer::Text));
        }
        if letters[pos] == Number {
            return Ok((Pos { value: nrs, end_index: pos }, Closer::Number));
        }
        let nr = if is_first {
            is_first = false;
            decode_uint_no_modifier_at_start(letters)?
        } else {
            decode_uint_no_modifier_at_start(&letters[pos..])?
        };
        debug_assert!(nr.end_index >= pos, "did not consume any letters while parsing uint_vec");
        pos = nr.end_index + 1;
        nrs.push(nr.value)
    }
}

#[cfg(test)]
mod encoding {
    use super::*;

    #[test]
    fn encode_empty_nr() {
        let enc = encode_uint_vec(&[], Closer::Number);
        assert_eq!(enc, vec![Text]);
    }

    #[test]
    fn encode_empty_txt() {
        let enc = encode_uint_vec(&[], Closer::Text);
        assert_eq!(enc, vec![Text]);
    }

    #[test]
    fn encode_single_nr() {
        let enc = encode_uint_vec(&[364], Closer::Number);
        assert_eq!(enc, vec![Asterisk, Bracket, Text, Number]);
    }

    #[test]
    fn encode_single_txt() {
        let enc = encode_uint_vec(&[364], Closer::Text);
        assert_eq!(enc, vec![Asterisk, Bracket, Text, Text]);
    }

    #[test]
    fn encode_examples_nr() {
        let enc = encode_uint_vec(&[44, 511, 0], Closer::Number);
        assert_eq!(enc, vec![Asterisk, Text, Io, Io, Io, Io, Colon, Bracket, Number]);
    }

    #[test]
    fn encode_examples_txt() {
        let enc = encode_uint_vec(&[44, 511, 0], Closer::Text);
        assert_eq!(enc, vec![Asterisk, Text, Io, Io, Io, Io, Colon, Bracket, Text]);
    }
}

#[cfg(test)]
mod decoding {
    use super::*;

    #[test]
    fn decode_empty_nr() {
        let enc = decode_uint_vec(&[Number]).unwrap();
        assert_eq!(enc.0.value.len(), 0);
        assert_eq!(enc.0.end_index, 0);
    }

    #[test]
    fn decode_empty_txt() {
        let enc = decode_uint_vec(&[Text]).unwrap();
        assert_eq!(enc.0.value, &[]);
        assert_eq!(enc.0.end_index, 0);
        assert_eq!(enc.1, Closer::Text);
    }

    #[test]
    fn decode_empty_noend() {
        let enc = decode_uint_vec(&[]).unwrap();
        assert_eq!(enc.0.value, &[]);
        assert_eq!(enc.0.end_index, 0);
        assert_eq!(enc.1, Closer::Text);
    }

    #[test]
    fn decode_single_nr() {
        let enc = decode_uint_vec(&[Asterisk, Bracket, Text, Number]).unwrap();
        assert_eq!(enc.0.value, &[364]);
        assert_eq!(enc.0.end_index, 1);
        assert_eq!(enc.1, Closer::Number);
    }

    #[test]
    fn decode_single_txt() {
        let enc = decode_uint_vec(&[Asterisk, Bracket, Text, Text]).unwrap();
        assert_eq!(enc.0.value, &[364]);
        assert_eq!(enc.0.end_index, 1);
        assert_eq!(enc.1, Closer::Text);
    }

    #[test]
    fn decode_examples_nr() {
        let enc = decode_uint_vec(&[Asterisk, Text, Io, Io, Io, Io, Colon, Bracket, Number]).unwrap();
        assert_eq!(enc.0.value, &[44, 511, 0]);
        assert_eq!(enc.0.end_index, 3);
        assert_eq!(enc.1, Closer::Number);
    }

    #[test]
    fn decode_examples_txt() {
        let enc = decode_uint_vec(&[Asterisk, Text, Io, Io, Io, Io, Colon, Bracket, Text]).unwrap();
        assert_eq!(enc.0.value, &[44, 511, 0]);
        assert_eq!(enc.0.end_index, 3);
        assert_eq!(enc.1, Closer::Text);
    }

    //TODO @mark: longer inputs
    //TODO @mark: missing end token
}
