use ::std::fmt;
use ::std::fmt::Formatter;
use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::compile::var_uint::encode_uint_allow_modifiers;
use crate::compile::var_uint::encode_uint_no_modifier_at_start;
use crate::compile::var_uint::DecodeError::TooLarge;
use crate::op::Op;
use crate::Value::Num;
use crate::NR;
use crate::UINT;

#[derive(Debug, Copy, Clone)]
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

pub fn decode_uint_vec(letters: &[Letter]) -> (Vec<UINT>, Closer) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
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
    fn encode_examples_nr() {
        unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
    }
}
