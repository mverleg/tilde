
//TODO @mark: $magic-newline-value$
//TODO @mark: 0 is reserved (for backspace)
//TODO @mark: fallback to full unicode after end?

use ::std::sync::{Arc, Mutex, RwLock};
use ::std::sync::LazyLock;
use ::std::slice::Iter;
use ::std::iter::Cloned;
use ::std::iter::FlatMap;

static RAW_DICT: &'static str = include_str!("../../dictionary.txt");
static DICT: LazyLock<DictContainer> = LazyLock::new(|| DictContainer::new());

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DictEntry {
    Snippet(&'static str),
    Backspace,
    CapitalizeFirst,
    CapitalizeAll,
}

struct DictContainer {
    snippet_lookup: Vec<DictEntry>,
}

#[derive(Clone)]
pub struct Dictionary {
}

impl DictContainer {
    fn new() -> Self {
        DictContainer {
            snippet_lookup: RAW_DICT.split("\n")
                .map(|line| match line {
                    "$magic-backspace$" => DictEntry::Backspace,
                    "$magic-newline$" => DictEntry::Snippet("\n"),
                    "$magic-capitalize-first$" => DictEntry::CapitalizeFirst,
                    "$magic-capitalize all$" => DictEntry::CapitalizeAll,
                    _ => DictEntry::Snippet(line),
                })
                .collect(),
        }
    }
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {}
    }

    pub fn index(&self, position: usize) -> Option<DictEntry> {
        DICT.snippet_lookup.get(position).copied()
    }

    fn iter(&self) -> impl Iterator<Item = DictEntry> {
        DICT.snippet_lookup.iter().cloned()
    }

    fn iter_snippets(&self) -> impl Iterator<Item = &'static str> {
        self.iter().flat_map(|entry| match entry {
            DictEntry::Snippet(snip) => Some(snip),
            _ => None,
        }).into_iter()
    }
}

#[cfg(test)]
mod lookup {
use ::std::collections::HashSet;
    use super::*;

    #[test]
    fn first_is_whitespace() {
        let dict = Dictionary::new();
        assert_eq!(dict.index(1), Some(DictEntry::Snippet(" ")), "first entry should be space (maybe stripped by editor?)");
    }

    #[test]
    fn trailing_whitespace() {
        let dict = Dictionary::new();
        let trailing_whitespace_count = dict.iter_snippets()
            .filter(|entry| entry.ends_with(" "))
            .count();
        assert!(trailing_whitespace_count > 10, "quite some entries should have trailing space (maybe stripped by editor?)");
    }

    #[test]
    fn no_duplicates() {
        let dict = Dictionary::new();
        let mut seen = HashSet::new();
        for entry in dict.iter() {
            assert!(seen.insert(entry), "duplicate: {entry:?}");
        }
    }

    #[test]
    fn parsed_all_specials() {
        let dict = Dictionary::new();
        for entry in dict.iter_snippets() {
            if entry.len() > 2 {
                assert!(!(entry.starts_with("$") && entry.ends_with("$")), "unparsed magic value: {entry:?}")
            }
        }
    }

    //TODO @mark: all special values encountered
}
