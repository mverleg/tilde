use ::std::fmt;
use ::std::fmt::Formatter;
use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::compile::var_uint::DecodeError::TooLarge;
use crate::op::Op;
use crate::NR;
use crate::UINT;

fn encode_uint_vec(nrs: &[UINT]) -> Vec<Letter> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn decode_uint_vec(letters: &[Letter]) -> Vec<UINT> {
    unimplemented!()
}
