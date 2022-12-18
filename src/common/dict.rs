//TODO: maybe make this a separate crate, perhaps together with variable encoding?

//TODO @mark: fallback to full unicode after end?

use ::std::borrow::Cow;
use ::std::collections::HashMap;
use ::std::iter::Cloned;
use ::std::iter::FlatMap;
use ::std::slice::Iter;
use ::std::sync::{Arc, Mutex, RwLock};
use ::std::sync::LazyLock;
use ::std::collections::HashSet;

use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;
use ::tinyvec::ArrayVec;

use crate::common::dict_derive::{cap_derivations, CapitalizeKind};
use crate::common::trie::Trie;

pub type INDX = u16;
//TODO @mark: unit test to see if index can be narrower?
pub type SnipCombi = ArrayVec<[INDX; 4]>;
//TODO @mark: unit test to see if array can be smaller

//TODO @mark: memoize hashes of dict entries with some wrapper

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
        let derivations = DERIVED_DICT.split("\n").collect::<HashSet<&'static str>>();
        let mut position_lookup = HashMap::with_capacity(derivations.len());
        for (pos, entry) in list.iter().enumerate() {
            generate_extended_snippet_combis(pos, entry, &derivations,
                                             |text, combi| { position_lookup.insert(text, combi); })
        }
        // let snippet_positions = list.iter().enumerate()
        //     .flat_map(|(pos, entry)| generate_extended_snippet_combis(pos, entry, &derivations))
        //     .collect::<HashMap<&'static str, SnipCombi>>();
        let mut trie = Trie::new();
        for text in position_lookup.keys() {
            trie.push(*text)
        }
        DictContainer {
            snippet_index: list,
            ext_snippet_positions: position_lookup,
            ext_prefix_tree: trie,
        }
    }
}

fn generate_extended_snippet_combis(
    pos: usize,
    entry: &DictEntry,
    derivations: &HashSet<&'static str>,
    mut entry_handler: impl FnMut(&'static str, SnipCombi),
) {
    let DictEntry::Snippet(base_snippet) = *entry else {
        return;
    };
    for cap_deriv in cap_derivations(base_snippet) {
        let deriv_text: &'static str = *derivations.get(cap_deriv.text.as_str()).expect("not found in pre-computed derivations");
        let deriv_ops: SnipCombi;
        entry_handler(deriv_text, deriv_ops);
        eprintln!("add backspaces")
    }
    //let q: INDX = pos.try_into().expect("positions exceeded INDX");
    //let immortal_snippet = derivations.get(base_snippet).expect("");
}

pub fn dict_iter() -> impl Iterator<Item=DictEntry> {
    DICT.snippet_index.iter().cloned()
}

pub fn dict_iter_snippets() -> impl Iterator<Item=&'static str> {
    dict_iter().flat_map(|entry| match entry {
        DictEntry::Snippet(snip) => Some(snip),
        _ => None,
    }).into_iter()
}

