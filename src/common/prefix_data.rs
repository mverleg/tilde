
// This files uses len_utf8 for char length, based on the promise that str is utf8
// https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8

//TODO @mverleg: is there a data structure with more efficient hashcode? arrayvec-specific map?

use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use ::std::collections::VecDeque;
use ::std::fmt::Debug;
use ::std::vec::IntoIter;
use ::std::hash;
use ::std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

use ::nohash_hasher::{BuildNoHashHasher, NoHashHasher};
use ::tinyvec_string::ArrayString;

use crate::common::INDX;
use crate::common::text_trans::{CowDictStr, DictStr, LONGEST_DICT_ENTRY_BYTES};
use crate::common::tiny_map::TinyMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrieLookup<'a, Word> {
    IsWord(&'a Word),
    NotFound,
}

#[derive(Debug, PartialEq, Eq)]
struct Key(DictStr);

impl hash::Hash for Key {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // Not resistant to pathological input, but the actual input is trusted and static.
        let bytes = self.0.as_bytes();
        let mut hash: u32 = 0;
        let mut i = 0;
        let u8size = u8::MAX as u32;
        let n = bytes.len();
        while i * 4 + 4 < n {
            hash += u8size * u8size * u8size * (bytes[i + 3] as u32);
            hash += u8size * u8size * (bytes[i + 2] as u32);
            hash += u8size * (bytes[i + 1] as u32);
            hash += bytes[i] as u32;
            i += 4;
        }
        if i + 2 < n {
            hash += u8size * u8size * (bytes[i + 2] as u32);
        }
        if i + 1 < n {
            hash += u8size * (bytes[i + 1] as u32);
        }
        if i < n {
            hash += bytes[i] as u32;
        }
        eprintln!("hash '{}' as {}", self.0.as_str(), hash);  //TODO @mark: TEMPORARY! REMOVE THIS!
        state.write_u32(hash)
    }
}

impl nohash_hasher::IsEnabled for Key {}

#[derive(Debug)]
pub struct PrefixMap<Word> {
    words: HashMap<Key, Word, BuildHasherDefault<NoHashHasher<Key>>>,
}

impl <Word: Debug> PrefixMap<Word> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        PrefixMap {
            words: HashMap::with_capacity_and_hasher(cap, BuildHasherDefault::default()),
        }
    }

    pub fn push(&mut self, text: DictStr, value: Word) {
        eprintln!("insert '{}'", text.as_str());  //TODO @mark:
        //debug_assert!(!self.words.contains_key(&Key(text)));
        //TODO @mark: maybe re-enable this assert if derivations get de-duplicated?
        self.words.insert(Key(text), value);
    }

    pub fn lookup<'a>(&'a self, value: &DictStr) -> TrieLookup<'a, Word> {
        match self.words.get(&Key(*value)) {
            Some(word) => TrieLookup::IsWord(word),
            None => TrieLookup::NotFound,
        }
    }

    pub fn contains_exactly(&self, value: &DictStr) -> bool {
        self.words.get(&Key(*value)).is_some()
    }
}

impl <Word: Clone + Debug> PrefixMap<Word> {

    #[cfg(test)]
    fn longest_prefix(&self, text: &str) -> Option<Word> {
        let mut buffer = Vec::new();
        self.all_prefixes_cloned_of(text, &mut buffer);
        buffer.into_iter().last()
    }

    #[cfg(test)]
    fn all_prefixes_of(&self, text: &str) -> Vec<Word> {
        let mut matches = Vec::new();
        self.all_prefixes_cloned_of(text, &mut matches);
        matches
    }

    /// Given a text, find all the words that are prefixes of it. E.g. "dogma" is ["do", "dog", "dogma"].
    pub fn all_prefixes_cloned_of(&self, text: &str, buffer: &mut Vec<Word>) {
        text
            .chars()
            .map(|c| c.len_utf8())
            .scan(0usize, |acc, add| {
                *acc += add;
                Some(*acc)
            })
            .take_while(|length| *length <= LONGEST_DICT_ENTRY_BYTES)
            .flat_map(|upto| {
                let key = DictStr::from(&text[0..upto]);
                //TODO @mverleg: lot of copying here, even though it's just on stack
                let value = self.words.get(&Key(key));
                value.map(|word| (*word).clone()).into_iter()
            })
            .for_each(|word| buffer.push(word))
    }
}

#[cfg(test)]
mod tests {
    use ::std::mem::size_of;

    use super::*;

    #[test]
    fn index_size() {
        assert!(size_of::<INDX>() <= size_of::<usize>(),
                "usize is smaller than index on this platform, this is not supported");
    }

    #[test]
    fn empty() {
        let trie = PrefixMap::<()>::new();
        assert!(!trie.contains_exactly(&DictStr::from("hello")));
    }

    #[test]
    fn build() {
        let mut trie = PrefixMap::new();
        trie.push(DictStr::from("hello"), 1);
        assert_eq!(trie.lookup(&DictStr::from("hello")), TrieLookup::IsWord(&1));
        assert_eq!(trie.lookup(&DictStr::from("he")), TrieLookup::NotFound);
        assert_eq!(trie.lookup(&DictStr::from("eh")), TrieLookup::NotFound);
        trie.push(DictStr::from("he"), 2);
        assert_eq!(trie.lookup(&DictStr::from("he")), TrieLookup::IsWord(&2));
        assert_eq!(trie.lookup(&DictStr::from("hel")), TrieLookup::NotFound);
        trie.push(DictStr::from("hell"), 3);
        assert_eq!(trie.lookup(&DictStr::from("hell")), TrieLookup::IsWord(&3));
        assert_eq!(trie.lookup(&DictStr::from("hel")), TrieLookup::NotFound);
        trie.push(DictStr::from("hey"), 4);
        assert_eq!(trie.lookup(&DictStr::from("hey")), TrieLookup::IsWord(&4));
        assert_eq!(trie.lookup(&DictStr::from("h")), TrieLookup::NotFound);
        assert_eq!(trie.lookup(&DictStr::from("p")), TrieLookup::NotFound);
        assert_eq!(trie.lookup(&DictStr::from("hello")), TrieLookup::IsWord(&1));
    }

    fn build_test_trie() -> PrefixMap<i8> {
        let mut trie = PrefixMap::new();
        trie.push(DictStr::from("hello"), 1);
        trie.push(DictStr::from("he"), 2);
        trie.push(DictStr::from("hell"), 3);
        trie.push(DictStr::from("help"), 4);
        trie.push(DictStr::from("hey"), 5);
        trie.push(DictStr::from("hero"), 6);
        trie.push(DictStr::from("helvetica"), 7);
        trie.push(DictStr::from("potato"), 8);
        trie
    }

    fn value_for<T: Clone + Debug>(trie: &PrefixMap<T>, text: &str) -> T {
        let TrieLookup::IsWord(word) = trie.lookup(&DictStr::from(text)) else {
            panic!("did not find {}", text)
        };
        (*word).clone()
    }

    #[test]
    fn longest_prefix_out_of_input_while_at_word() {
        let mut trie = build_test_trie();
        assert_eq!(trie.longest_prefix("hell").unwrap(), value_for(&trie, "hell"));
    }

    #[test]
    fn longest_prefix_out_of_input_while_not_at_word() {
        let mut trie = build_test_trie();
        assert_eq!(trie.longest_prefix("her").unwrap(), value_for(&trie, "he"));
    }

    #[test]
    fn longest_prefix_out_of_matches_while_deepest_is_word() {
        let mut trie = build_test_trie();
        assert_eq!(trie.longest_prefix("helpless").unwrap(), value_for(&trie, "help"));
    }

    #[test]
    fn longest_prefix_out_of_matches_while_deepest_is_not_word() {
        let mut trie = build_test_trie();
        assert_eq!(trie.longest_prefix("helve").unwrap(), value_for(&trie, "he"));
    }

    #[test]
    fn longest_prefix_unknown_prefix() {
        let mut trie = build_test_trie();
        assert!(trie.longest_prefix("abacus").is_none());
    }

    #[test]
    fn test_all_prefixes_of_no_match() {
        let mut trie = build_test_trie();
        assert!(trie.all_prefixes_of("abacus").is_empty());
    }

    #[test]
    fn test_all_prefixes_of_exact_match() {
        let mut trie = build_test_trie();
        assert_eq!(trie.all_prefixes_of("hell"),
            vec![value_for(&trie, "he"), value_for(&trie, &DictStr::from("hell"))]);
    }

    #[test]
    fn test_all_prefixes_of_sub_matches() {
        let mut trie = build_test_trie();
        assert_eq!(trie.all_prefixes_of("helpless"),
            vec![value_for(&trie, "he"), value_for(&trie, &DictStr::from("help"))]);
    }
}
