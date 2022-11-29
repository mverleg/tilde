
//TODO @mark: fallback to full unicode after end?

use ::std::iter::Cloned;
use ::std::iter::FlatMap;
use ::std::slice::Iter;
use ::std::sync::{Arc, Mutex, RwLock};
use ::std::sync::LazyLock;

use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

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
    pub fn snip(text: &'static str, capitalize_next: bool) -> Self {
        DictEntry::Snippet { text, capitalize_next }
    }
}

struct DictContainer {
    snippet_lookup: Vec<DictEntry>,
}

impl DictContainer {
    fn new() -> Self {
        DictContainer {
            snippet_lookup: RAW_DICT.split("\n")
                .map(|line| match line {
                    "$magic-backspace$" => DictEntry::Backspace,
                    "$magic-newline$" => DictEntry::snip("\n", true),
                    "$magic-capitalize-first$" => DictEntry::CapitalizeFirst,
                    "$magic-capitalize all$" => DictEntry::CapitalizeAll,
                    _ => if line.ends_with("$capitalize-next$") {
                        DictEntry::snip(line.strip_suffix("$capitalize-next$").unwrap(), true)
                    } else {
                        DictEntry::snip(line, false)
                    },
                })
                .collect(),
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

#[cfg(test)]
mod lookup {
    use ::std::collections::HashSet;

    use super::*;

    #[test]
    fn first_is_whitespace() {
        assert_eq!(dict_index(1), Some(DictEntry::snip(" ", false)), "first entry should be space (maybe stripped by editor?)");
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
