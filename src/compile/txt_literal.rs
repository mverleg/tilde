use ::std::fmt;
use ::std::fmt::Formatter;
use ::strum::IntoEnumIterator;

use crate::compile::letter::Letter;
use crate::compile::letter::Letter::*;
use crate::compile::var_uint::DecodeError::TooLarge;
use crate::op::Op;
use crate::NR;

#[inline]
fn encode_uint_vec(nrs: &[u64]) -> Vec<Letter> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn decode_uint_vec(letters: &[Letter]) -> Vec<u64> {
    unimplemented!()
}
