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
    let mut current_snip = "";
    let mut current_capitalize_next = false;
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
                transform.case_first = current_capitalize_next;
                current_capitalize_next = capitalize_next;
                //TODO @mark: do not count the capitalize next if it doesn't do anything? like on whitespace
            }
            DictEntry::Backspace => {
                transform.pop_end += 1;
            }
            DictEntry::CapitalizeFirst => {
                transform.case_first = !transform.case_first;
            }
            DictEntry::CapitalizeAll => {
                transform.case_all = !transform.case_all;
            }
        }
    }
    buffer.push_str(transform.apply(current_snip).as_ref());
}

include!(concat!(env!("OUT_DIR"), "/dict_init.rs"));

#[cfg(test)]
mod tests {
    use crate::common::text_trans::LONGEST_DICT_ENTRY_BYTES;

    use super::*;

    #[test]
    fn dict_index_width() {
        assert!((DICT.len() as u64) < (INDX::MAX as u64));
    }

    #[test]
    fn longest_dict_entry_const() {
        let longest_dict_entry = DICT.iter()
            .map(|entry| match entry {
                DictEntry::Snippet { snip, .. } => snip.len(),
                _ => 0,
            })
            .max().unwrap();
        assert_eq!(longest_dict_entry, LONGEST_DICT_ENTRY_BYTES);
    }

    #[test]
    fn lookup_simple() {
        let mut out = String::new();
        lookup_buffer(&[9, 2, 12, 12, 5, 1, 224], &mut out, &mut vec![]);
        assert_eq!(&out, "hello world ")
    }

    #[test]
    fn lookup_with_magic() {
        let mut out = String::new();
        lookup_buffer(&[89, 70, 2542, 0, 836, 0, 62, 754, 0], &mut out, &mut vec![]);
        assert_eq!(&out, "Asterisk! Capital")
    }
}
