use ::std::convert::TryInto;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::iter::Iterator;

use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::compile::parse::Pos;
use crate::compile::var_uint::decode_uint_allow_modifiers;
use crate::compile::var_uint::decode_uint_no_modifier_at_start;
use crate::compile::var_uint::DecodeError;
use crate::compile::var_uint::DecodeError::TooLarge;
use crate::compile::var_uint::encode_uint_allow_modifiers;
use crate::compile::var_uint::encode_uint_no_modifier_at_start;
use crate::dict::{compress_with_dict, DictIx, lookup_buffer};
use crate::{NR, TildeRes};
use crate::op::Op;
use crate::tilde_log;
use crate::UINT;
use crate::Value::Num;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Closer {
    Text,
    Number,
}

///
/// Note: the first letter cannot be a modifier or number token, so if the length is zero
/// and the closer is supposed to be number, we have too pick text token instead.
pub fn encode_uint_vec<I>(
    nrs: &[I],
    closer: Closer,
) -> Vec<Letter> where I: Into<UINT> + Copy {
    if nrs.is_empty() {
        return vec![Text];
    }
    let mut letters = vec![];
    letters.extend(encode_uint_no_modifier_at_start(nrs[0].into()));
    for nr in nrs.iter().skip(1) {
        let nr: UINT = (*nr).into();
        letters.extend(encode_uint_allow_modifiers(nr));
    }
    letters.push(match closer {
        Closer::Text => Text,
        Closer::Number => Number,
    });
    letters
}

///
/// String buffer is NOT cleared (can expand), decode buffer IS overwritten.
pub fn decode_str(letters: &[Letter], string_buffer: &mut String, decode_buffer: &mut Vec<UINT>) -> Result<Pos<()>, DecodeError> {
    let closer = decode_uint_vec_buffer(letters, decode_buffer)?;
    assert!(Closer::Text == closer.value);  //TODO @mark: for now only decodes strings, not int arrays
    let indices = decode_buffer.iter_mut().map(|nr| *nr as DictIx).collect::<Vec<_>>();
    lookup_buffer(&indices, string_buffer);
    Ok(Pos { value: (), length: closer.length })
}

pub fn decode_uint_vec(letters: &[Letter]) -> Result<(Pos<Vec<UINT>>, Closer), DecodeError> {
    let mut buffer: Vec<UINT> = Vec::new();
    let closer = decode_uint_vec_buffer(letters, &mut buffer)?;
    Ok((Pos {
        value: buffer,
        length: closer.length,
    }, closer.value))
}

pub fn encode_str(text: &str) -> TildeRes<Vec<Letter>> {
    let compress_ops = &compress_with_dict(text);
    let mut encoding = vec![Letter::Text];
    encoding.extend(encode_uint_vec(compress_ops, Closer::Text));
    //TODO @mark: no allocation?
    Ok(encoding)
}

//TODO @mark: buffer version used?
pub fn decode_uint_vec_buffer(letters: &[Letter], nrs_buffer: &mut Vec<UINT>) -> Result<Pos<Closer>, DecodeError> {
    let mut is_first = true;
    let mut pos = 0;
    nrs_buffer.clear();
    loop {
        if pos >= letters.len() {
            tilde_log!("uint_vec without end marker, interpreting as text");
            return Ok(Pos { value: Closer::Text, length: pos });
        }
        if letters[pos] == Text {
            return Ok(Pos { value: Closer::Text, length: pos + 1 });
        }
        if letters[pos] == Number {
            return Ok(Pos { value: Closer::Number, length: pos + 1 });
        }
        let nr = if is_first {
            is_first = false;
            decode_uint_no_modifier_at_start(letters)?
        } else {
            decode_uint_allow_modifiers(&letters[pos..])?
        };
        debug_assert!(nr.length > 0, "did not consume any letters while parsing uint_vec");
        pos += nr.length;
        nrs_buffer.push(nr.value)
    }
}

#[cfg(test)]
mod encoding {
    use super::*;

    #[test]
    fn encode_empty_nr() {
        let enc = encode_uint_vec::<UINT>(&[], Closer::Number);
        assert_eq!(enc, vec![Text]);
    }

    #[test]
    fn encode_empty_txt() {
        let enc = encode_uint_vec::<UINT>(&[], Closer::Text);
        assert_eq!(enc, vec![Text]);
    }

    #[test]
    fn encode_single_nr() {
        let enc = encode_uint_vec::<UINT>(&[364], Closer::Number);
        assert_eq!(enc, vec![Asterisk, Bracket, Text, Number]);
    }

    #[test]
    fn encode_single_txt() {
        let enc = encode_uint_vec::<UINT>(&[364], Closer::Text);
        assert_eq!(enc, vec![Asterisk, Bracket, Text, Text]);
    }

    #[test]
    fn encode_examples_nr() {
        let enc = encode_uint_vec::<UINT>(&[44, 511, 0], Closer::Number);
        assert_eq!(enc, vec![Asterisk, Text, Io, Io, Io, Io, Colon, Bracket, Number]);
    }

    #[test]
    fn encode_examples_txt() {
        let enc = encode_uint_vec::<UINT>(&[44, 511, 0], Closer::Text);
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
        assert_eq!(enc.0.length, 1);
    }

    #[test]
    fn decode_empty_txt() {
        let enc = decode_uint_vec(&[Text]).unwrap();
        assert_eq!(enc.0.value, &[]);
        assert_eq!(enc.0.length, 1);
        assert_eq!(enc.1, Closer::Text);
    }

    #[test]
    fn decode_empty_noend() {
        let enc = decode_uint_vec(&[]).unwrap();
        assert_eq!(enc.0.value, &[]);
        assert_eq!(enc.0.length, 0);
        assert_eq!(enc.1, Closer::Text);
    }

    #[test]
    fn decode_single_nr() {
        let enc = decode_uint_vec(&[Asterisk, Bracket, Text, Number]).unwrap();
        assert_eq!(enc.0.value, &[364]);
        assert_eq!(enc.0.length, 4);
        assert_eq!(enc.1, Closer::Number);
    }

    #[test]
    fn decode_single_txt() {
        let enc = decode_uint_vec(&[Asterisk, Bracket, Text, Text]).unwrap();
        assert_eq!(enc.0.value, &[364]);
        assert_eq!(enc.0.length, 4);
        assert_eq!(enc.1, Closer::Text);
    }

    #[test]
    fn decode_examples_nr() {
        let enc = decode_uint_vec(&[Asterisk, Text, Io, Io, Io, Io, Colon, Bracket, Number]).unwrap();
        assert_eq!(enc.0.value, &[44, 511, 0]);
        assert_eq!(enc.0.length, 9);
        assert_eq!(enc.1, Closer::Number);
    }

    #[test]
    fn decode_examples_txt() {
        let enc = decode_uint_vec(&[Asterisk, Text, Io, Io, Io, Io, Colon, Bracket, Text]).unwrap();
        assert_eq!(enc.0.value, &[44, 511, 0]);
        assert_eq!(enc.0.length, 9);
        assert_eq!(enc.1, Closer::Text);
    }

    #[test]
    fn decode_short() {
        let enc = decode_uint_vec(&[Slash, Colon, Hat, Exclamation, Question, Hash, Tilde]).unwrap();
        assert_eq!(enc.0.value, &[0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(enc.0.length, 7);
        assert_eq!(enc.1, Closer::Text);
    }

    #[test]
    fn decode_complex() {
        let enc = decode_uint_vec(&[Io, Seq, More, Colon, Plus, Hat, Asterisk,
            Exclamation, Question, Slash, Hash, Tilde, Number, Asterisk, Seq, More, Text, Plus, Hat,
            Bracket, Exclamation, Question, Slash, Hash, Tilde, Number, Number]).unwrap();
        assert_eq!(enc.0.value, &[18_873_338_202_050, 26_422_676_238_522]);
        assert_eq!(enc.0.length, 27);
        assert_eq!(enc.1, Closer::Number);
    }

    #[test]
    fn decode_no_end_token() {
        let res = decode_uint_vec(&[Slash, Colon, Hat, Io,]);
        assert!(res.is_err())
    }
}
