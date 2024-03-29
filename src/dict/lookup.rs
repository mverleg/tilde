use crate::common::{SnipOrChar, TextTransformation};
use crate::dict::{DICT, DictIx};
use crate::dict::entries::DictEntry;

pub fn lookup_alloc(indices: &[DictIx]) -> String {
    let mut buffer = String::new();
    lookup_buffer(indices, &mut buffer);
    buffer
}

struct LatestSnippet {
    indx: DictIx,
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

/// String buffer is NOT cleared (can expand)
pub fn lookup_buffer(indices: &[DictIx], buffer: &mut String) {
    let mut current = LatestSnippet { indx: 0, snip: "" };
    let mut current_capitalize_next = false;
    let mut transform = TextTransformation::new_noop();
    let mut is_unicode = false;
    for indx in indices {
        match DICT[*indx as usize] {
            DictEntry::Snippet { snip, capitalize_next, cost: _ } => {
                buffer.push_str(transform.apply(current.into_str(is_unicode)).as_ref());
                current = LatestSnippet { indx: *indx, snip };
                transform = TextTransformation::new_noop();
                is_unicode = false;
                transform.case_first = current_capitalize_next;
                current_capitalize_next = capitalize_next;
                //tilde_log!("snip = {} ({})", &current.snip, current.indx);
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
    use crate::dict::{DICT, DictIx};
    use crate::dict::dict_str::LONGEST_DICT_ENTRY_BYTES;
    use crate::dict::entries::DictEntry;

    use super::*;

    #[test]
    fn dict_index_width() {
        assert!((DICT.len() as u64) < (DictIx::MAX as u64));
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
        lookup_buffer(&[9, 2, 12, 12, 5, 1, 225], &mut out);
        assert_eq!(&out, "hello world ")
    }

    #[test]
    fn lookup_unicode() {
        let mut out = String::new();
        lookup_buffer(&[20320, 70, 22909, 70], &mut out);
        assert_eq!(&out, "你好")
    }

    #[test]
    fn lookup_with_magic() {
        // as-CAP/tea-BS-BS/risk-BS/!/capital-CAP-BS
        let mut out = String::new();
        lookup_buffer(&[90, 71, 2546, 0, 0, 840, 0, 62, 758, 0], &mut out);
        assert_eq!(&out, "Asterisk! Capital")
    }
}
