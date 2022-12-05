
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

use ::smallvec::SmallVec;

use crate::common::dict_derive::CapitalizeKind;
use crate::common::trie::Trie;
use crate::UINT;

type SnipCombi = SmallVec<[usize; 4]>;

static RAW_DICT: &'static str = include_str!("../../dictionary.txt");
static DERIVED_DICT: &'static str = include_str!(concat!(env!("OUT_DIR"), "/dictionary_extended.txt"));
pub(crate) static DICT: LazyLock<DictContainer> = LazyLock::new(|| DictContainer::new());

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum DictEntry {
    Snippet(&'static str),
    Backspace,
    CapitalizeFirst,
    CapitalizeAll,
}

impl DictEntry {
    pub fn new_snippet(text: &'static str, capitalize_next: bool) -> Self {
        DictEntry::Snippet(text)
    }

    pub fn get_snippet(&self) -> Option<&'static str> {
        match self {
            DictEntry::Snippet(snip) => Some(*snip),
            _ => None,
        }
    }
}

pub(crate) struct DictContainer {
    /// Find snippets by index in the raw dictionary.
    pub snippet_index: Vec<DictEntry>,
    /// Find positions of the (multiple) operations to create a derived snippet.
    pub ext_snippet_positions: HashMap<&'static str, SnipCombi>,
    //TODO @mark: optimize smallvec size
    /// Prefix-tree (trie) of all derived snippets.
    pub ext_prefix_tree: Trie,
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
        let position_lookup = todo!();  //TODO @mark: TEMPORARY! REMOVE THIS!
        DictContainer {
            snippet_index: list,
            ext_snippet_positions: position_lookup,
            ext_prefix_tree: trie,
        }
    }
}

pub fn dict_iter() -> impl Iterator<Item = DictEntry> {
    DICT.snippet_index.iter().cloned()
}

pub fn dict_iter_snippets() -> impl Iterator<Item = &'static str> {
    dict_iter().flat_map(|entry| match entry {
        DictEntry::Snippet(snip) => Some(snip),
        _ => None,
    }).into_iter()
}

