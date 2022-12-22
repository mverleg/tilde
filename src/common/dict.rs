#![allow(warnings)]
#![allow(clippy::all)]

//TODO @mark: add a pop-first operation
//TODO @mark: maybe a reverse operation?

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
    char_buffer.clear();
    let mut current_capitalize_next = true;
    for indx in indices {
        match DICT[*indx as usize] {
            DictEntry::Snippet { snip, capitalize_next } => {
                for chr in char_buffer.iter() {
                    buffer.push(*chr);
                }
                char_buffer.clear();
                for chr in snip.chars() {
                    char_buffer.push(chr);
                }
                current_capitalize_next = capitalize_next;
            }
            DictEntry::Backspace => {
                char_buffer.pop();
            }
            DictEntry::CapitalizeFirst => {
                let Some(orig_first) = char_buffer.first_mut() else {
                    continue
                };
                let mut upper = orig_first.to_uppercase();
                match upper.nth(0) {
                    Some(switch_first) => {
                        if switch_first != *orig_first {
                            assert!(upper.nth(1).is_none(), "multi-char uppercase representations not yet supported");  //TODO @mark
                            *orig_first = switch_first;
                            continue;
                        }
                    },
                    None => {}
                };
                let mut lower = orig_first.to_uppercase();
                match lower.nth(0) {
                    Some(switch_first) => {
                        if switch_first != *orig_first {
                            assert!(lower.nth(1).is_none(), "multi-char lowercase representations not yet supported");  //TODO @mark
                            *orig_first = switch_first;
                            continue;
                        }
                    },
                    None => {}
                }
            }
            DictEntry::CapitalizeAll => {}
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
