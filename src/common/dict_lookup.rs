use ::std::process::Output;

use crate::common::{INDX, TextTransformation};
use crate::common::dict::{DICT, DictEntry};
use crate::common::dict_str::{CowDictStr, DictStr};
use crate::common::text_trans::SnipOrChar;
use crate::tilde_log;

pub fn lookup_alloc(indices: &[INDX]) -> String {
    let mut buffer = String::new();
    lookup_buffer(indices, &mut buffer, &mut vec![]);
    buffer
}

struct LatestSnippet {
    indx: INDX,
    snip: &'static str,
}

impl LatestSnippet {
    fn into_str(self, is_unicode: bool) -> SnipOrChar {
        if is_unicode {
            SnipOrChar::Char(char::from_u32(self.indx as u32)
                .unwrap_or_else(|| panic!("tried to create unicode entry #{} but failed", self.indx)))
            //TODO @mark: u16 does not cover most of unicode, switch to u32 (which does)?
            //TODO @mark: should this error case be handled? it can happen for quite some numbers
        } else {
            SnipOrChar::Snip(self.snip)
        }
    }
}

///
/// String buffer is NOT cleared (can expand), char buffer IS overwritten.
pub fn lookup_buffer(indices: &[INDX], buffer: &mut String, char_buffer: &mut Vec<char>) {
    //TODO @mark: remove `char_buffer` arg and rustdoc
    let mut current = LatestSnippet { indx: 0, snip: "" };
    let mut current_capitalize_next = false;
    let mut transform = TextTransformation::new_noop();
    let mut is_unicode = false;
    for indx in indices {
        match DICT[*indx as usize] {
            DictEntry::Snippet { snip, capitalize_next } => {
                buffer.push_str(transform.apply(current.into_str(is_unicode)).as_ref());
                current = LatestSnippet { indx: *indx, snip };
                transform = TextTransformation::new_noop();
                is_unicode = false;
                transform.case_first = current_capitalize_next;
                current_capitalize_next = capitalize_next;
                tilde_log!("snip = {} ({})", &current.snip, current.indx);
                //TODO @mark: do not count the capitalize next if it doesn't do anything? like on whitespace
            }
            DictEntry::UnicodeLookup => {
                is_unicode = true
            }
            DictEntry::Backspace => {
                transform.pop_end += 1;
            }
            DictEntry::BackspaceFront => {
                transform.pop_start += 1;
            }
            DictEntry::CapitalizeFirst => {
                transform.case_first = !transform.case_first;
            }
            DictEntry::CapitalizeAll => {
                transform.case_all = !transform.case_all;
            }
            DictEntry::Reverse => {
                transform.reverse = !transform.reverse;
            }
        }
    }
    buffer.push_str(transform.apply(current.into_str(is_unicode)).as_ref());
}

#[cfg(test)]
mod tests {
    use crate::common::dict_str::LONGEST_DICT_ENTRY_BYTES;

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
        lookup_buffer(&[9, 2, 12, 12, 5, 1, 225], &mut out, &mut vec![]);
        assert_eq!(&out, "hello world ")
    }

    #[test]
    fn lookup_unicode() {
        let mut out = String::new();
        lookup_buffer(&[20320, 70, 22909, 70], &mut out, &mut vec![]);
        assert_eq!(&out, "你好")
    }

    #[test]
    fn lookup_with_magic() {
        // as-CAP/tea-BS-BS/risk-BS/!/capital-CAP-BS
        let mut out = String::new();
        lookup_buffer(&[90, 71, 2545, 0, 0, 839, 0, 62, 757, 0], &mut out, &mut vec![]);
        assert_eq!(&out, "Asterisk! Capital")
    }
}
