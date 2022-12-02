
//TODO: maybe make this a separate crate, perhaps together with variable encoding?

//TODO @mark: fallback to full unicode after end?

use ::std::collections::HashMap;
use ::std::iter::Cloned;
use ::std::iter::FlatMap;
use ::std::slice::Iter;
use ::std::sync::{Arc, Mutex, RwLock};
use ::std::sync::LazyLock;

use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

use crate::common::trie::Trie;
use crate::UINT;

static RAW_DICT: &'static str = include_str!("../../dictionary.txt");
static DICT: LazyLock<DictContainer> = LazyLock::new(|| DictContainer::new());

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum DictEntry {
    Snippet { text: &'static str, capitalize_next: bool },
    Backspace,
    CapitalizeFirst,
    CapitalizeAll,
}

impl DictEntry {
    pub fn new_snippet(text: &'static str, capitalize_next: bool) -> Self {
        DictEntry::Snippet { text, capitalize_next }
    }

    pub fn get_snippet(&self) -> Option<&'static str> {
        match self {
            DictEntry::Snippet { text: snip, capitalize_next: _ } => Some(*snip),
            _ => None,
        }
    }
}

struct DictContainer {
    snippet_lookup: Vec<DictEntry>,
    position_lookup: HashMap<&'static str, usize>,
    prefix_tree: Trie,
}

impl DictContainer {
    fn new() -> Self {
        let list: Vec<DictEntry> = RAW_DICT.split("\n")
            .map(|line| match line {
                "$magic-backspace$" => DictEntry::Backspace,
                "$magic-newline$" => DictEntry::new_snippet("\n", true),
                "$magic-capitalize-first$" => DictEntry::CapitalizeFirst,
                "$magic-capitalize all$" => DictEntry::CapitalizeAll,
                _ => if line.ends_with("$capitalize-next$") {
                    DictEntry::new_snippet(line.strip_suffix("$capitalize-next$").unwrap(), true)
                } else {
                    DictEntry::new_snippet(line, false)
                },
            })
            .collect();
        let position_lookup = list.iter().enumerate()
            .flat_map(|(pos, entry)| entry.get_snippet().map(|text| (text, pos)).into_iter())
            .collect::<HashMap<&'static str, usize>>();
        let mut trie = Trie::new();
        for (text, _) in &position_lookup {
            trie.push(*text)
        }
        DictContainer {
            snippet_lookup: list,
            position_lookup: position_lookup,
            prefix_tree: trie,
        }
    }
}

pub fn dict_index(position: usize) -> Option<DictEntry> {
    DICT.snippet_lookup.get(position).copied()
}

fn dict_iter() -> impl Iterator<Item = DictEntry> {
    DICT.snippet_lookup.iter().cloned()
}

fn dict_iter_snippets() -> impl Iterator<Item = &'static str> {
    dict_iter().flat_map(|entry| match entry {
        DictEntry::Snippet { text: snip, capitalize_next: _ } => Some(snip),
        _ => None,
    }).into_iter()
}

pub fn compress_with_dict(text: &str) -> Vec<UINT> {
    let mut rem = text;
    let mut nrs = vec![];
    while !rem.is_empty() {
        let prefix = DICT.prefix_tree.longest_prefix(rem);
        if prefix.is_empty() {
            //TODO @mark: return Err instead of panic?
            panic!("cannot encode string because dictionary does not contain '{}'", rem.chars().next().unwrap())
        }
        rem = &rem[prefix.len()..];
        let nr = *DICT.position_lookup.get(prefix.as_str()).expect("prefix not in dictionary") as UINT;
        nrs.push(nr)
    }
    nrs
}

#[cfg(test)]
mod lookup {
    use ::std::collections::HashSet;

    use super::*;

    #[test]
    fn first_is_whitespace() {
        assert_eq!(dict_index(1), Some(DictEntry::new_snippet(" ", false)), "first entry should be space (maybe stripped by editor?)");
    }

    #[test]
    fn trailing_whitespace() {
        let trailing_whitespace_count = dict_iter_snippets()
            .filter(|entry| entry.ends_with(" "))
            .count();
        assert!(trailing_whitespace_count > 10, "quite some entries should have trailing space (maybe stripped by editor?)");
    }

    #[test]
    fn no_duplicates() {
        let mut seen = HashSet::new();
        for entry in dict_iter() {
            assert!(seen.insert(entry), "duplicate: {entry:?}");
        }
    }

    #[test]
    fn no_leftover_specials() {
        for entry in dict_iter_snippets() {
            if entry.matches("$").count() >= 2 {
                panic!("unparsed magic value: {entry:?}")
            }
        }
    }

    #[test]
    fn all_specials_encountered() {
        let seen = dict_iter()
            .filter(|entry| !matches!(entry, DictEntry::Snippet { .. }))
            .collect::<HashSet<_>>();
        for expect in DictEntry::iter() {
            if matches!(expect, DictEntry::Snippet { .. }) {
                continue
            }
            assert!(seen.contains(&expect), "expected in dict: {expect:?}");
        }
    }
}

#[cfg(test)]
mod compression {
    use super::*;

    #[test]
    fn simple_text_compression() {
        let nrs = compress_with_dict("hello world, this is a test");
        assert_eq!(nrs.len(), 16);
        //TODO @mark: test more
    }
}
