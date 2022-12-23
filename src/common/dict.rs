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

const fn s(snip: &'static str) -> DictEntry {
    DictEntry::Snippet{ snip, capitalize_next: false, }
}
//noinspection RsFunctionNaming
const fn S(snip: &'static str) -> DictEntry {
    DictEntry::Snippet{ snip, capitalize_next: true, }
}

///
/// String buffer is NOT cleared (can expand), char buffer IS overwritten.
pub fn lookup_buffer(indices: &[INDX], buffer: &mut String, char_buffer: &mut Vec<char>) {
    //TODO @mark: remove `char_buffer` arg and rustdoc
    let mut current_capitalize_next = true;
    let mut current_snip = "";
    let mut transform = TextTransformation::new_noop();
    for indx in indices {
        // if current_capitalize_next {
        //     transform.case_first = true;
        //     current_capitalize_next = false;
        // }
        match DICT[*indx as usize] {
            DictEntry::Snippet { snip, capitalize_next } => {
                buffer.push_str(transform.apply(current_snip).as_ref());
                current_snip = snip;
                transform = TextTransformation::new_noop();
            }
            DictEntry::Backspace => {
                transform.pop_end += 1;
            }
            DictEntry::CapitalizeFirst => {
                transform.case_first = true;
            }
            DictEntry::CapitalizeAll => {
                transform.case_all = true;
            }
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/dict_init.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dict_index_width() {
        assert!((DICT.len() as u64) < (INDX::MAX as u64));
    }
}
