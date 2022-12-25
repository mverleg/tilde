#![allow(warnings)]
#![allow(clippy::all)]

//TODO @mark: add a pop-first operation
//TODO @mark: maybe a reverse operation?

use crate::common::TextTransformation;

pub type INDX = u16;

#[derive(Debug, Clone, Copy)]
pub enum DictEntry {
    Snippet { snip: &'static str, capitalize_next: bool, },
    Backspace,
    CapitalizeFirst,
    CapitalizeAll,
}

#[derive(Debug)]
pub struct DerivationInfo {
    pub derived_text: &'static str,
    pub original_index: usize,
    pub transformation: TextTransformation,
    pub cost: usize,
}

#[inline]
const fn s(snip: &'static str) -> DictEntry {
    DictEntry::Snippet{ snip, capitalize_next: false, }
}
//noinspection RsFunctionNaming
#[inline]
const fn S(snip: &'static str) -> DictEntry {
    DictEntry::Snippet{ snip, capitalize_next: true, }
}

include!(concat!(env!("OUT_DIR"), "/dict_init.rs"));
