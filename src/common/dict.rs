
//TODO: maybe make this a separate crate, perhaps together with variable encoding?

//TODO @mark: fallback to full unicode after end?

use ::std::borrow::Cow;
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
pub(crate) static DICT: LazyLock<DictContainer> = LazyLock::new(|| DictContainer::new());

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum DictEntry {
    Snippet { text: &'static str, capitalize_next: bool },
    Backspace,
    CapitalizeFirst,
    CapitalizeAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum CapitalizeKind {
    None,
    First,
    All,
}
//TODO @mark: used?

#[derive(Debug, Clone)]
pub struct DictCombi {
    text: String,
    base_snippet: &'static str,
    capitalize_self: CapitalizeKind,
    capitalize_next: bool,
    backspaced: u8,
    cost: u8,
}
//TODO @mark: used?

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

pub(crate) struct DictContainer {
    pub snippet_index: Vec<DictEntry>,
    pub snippet_positions: HashMap<&'static str, usize>,
    pub prefix_tree: Trie,
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
            snippet_index: list,
            snippet_positions: position_lookup,
            prefix_tree: trie,
        }
    }
}

pub fn dict_iter() -> impl Iterator<Item = DictEntry> {
    DICT.snippet_index.iter().cloned()
}

pub fn dict_iter_snippets() -> impl Iterator<Item = &'static str> {
    dict_iter().flat_map(|entry| match entry {
        DictEntry::Snippet { text: snip, capitalize_next: _ } => Some(snip),
        _ => None,
    }).into_iter()
}

