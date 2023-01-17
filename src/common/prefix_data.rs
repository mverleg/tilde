
// This files uses len_utf8 for char length, based on the promise that str is utf8
// https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8

//TODO @mverleg: is there a data structure with more efficient hashcode? arrayvec-specific map?

use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use ::std::collections::VecDeque;
use ::std::fmt::Debug;
use ::std::hash;
use ::std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};
use ::std::vec::IntoIter;

use ::fnv::{FnvBuildHasher, FnvHashMap};
use ::tinyvec_string::ArrayString;

use crate::common::dict_str::{DictStr, LONGEST_DICT_ENTRY_BYTES};
use crate::common::INDX;
use crate::common::tiny_map::TinyMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrefixMapLookup<'a, Word> {
    IsWord(&'a Word),
    NotFound,
}

#[derive(Debug)]
pub struct PrefixMap<Word> {
    words: HashMap<DictStr, Word>,
}

impl <Word: Debug> PrefixMap<Word> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        PrefixMap {
            words: HashMap::with_capacity(cap),
        }
    }

    pub fn push(&mut self, text: DictStr, value: Word) {
        //debug_assert!(!self.words.contains_key(&Key(text)));
        //TODO @mark: maybe re-enable this assert if derivations get de-duplicated?
        self.words.insert(text, value);
    }

    pub fn lookup<'a>(&'a self, value: &DictStr) -> PrefixMapLookup<'a, Word> {
        match self.words.get(&*value) {
            Some(word) => PrefixMapLookup::IsWord(word),
            None => PrefixMapLookup::NotFound,
        }
    }

    pub fn contains_exactly(&self, value: &DictStr) -> bool {
        self.words.get(&*value).is_some()
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
                let value = self.words.get(&key);
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
        let pm = PrefixMap::<()>::new();
        assert!(!pm.contains_exactly(&DictStr::from("hello")));
    }

    #[test]
    fn build() {
        let mut pm = PrefixMap::new();
        pm.push(DictStr::from("hello"), 1);
        assert_eq!(pm.lookup(&DictStr::from("hello")), PrefixMapLookup::IsWord(&1));
        assert_eq!(pm.lookup(&DictStr::from("he")), PrefixMapLookup::NotFound);
        assert_eq!(pm.lookup(&DictStr::from("eh")), PrefixMapLookup::NotFound);
        pm.push(DictStr::from("he"), 2);
        assert_eq!(pm.lookup(&DictStr::from("he")), PrefixMapLookup::IsWord(&2));
        assert_eq!(pm.lookup(&DictStr::from("hel")), PrefixMapLookup::NotFound);
        pm.push(DictStr::from("hell"), 3);
        assert_eq!(pm.lookup(&DictStr::from("hell")), PrefixMapLookup::IsWord(&3));
        assert_eq!(pm.lookup(&DictStr::from("hel")), PrefixMapLookup::NotFound);
        pm.push(DictStr::from("hey"), 4);
        assert_eq!(pm.lookup(&DictStr::from("hey")), PrefixMapLookup::IsWord(&4));
        assert_eq!(pm.lookup(&DictStr::from("h")), PrefixMapLookup::NotFound);
        assert_eq!(pm.lookup(&DictStr::from("p")), PrefixMapLookup::NotFound);
        assert_eq!(pm.lookup(&DictStr::from("hello")), PrefixMapLookup::IsWord(&1));
    }

    fn build_test_prefix_map() -> PrefixMap<i8> {
        let mut pm = PrefixMap::new();
        pm.push(DictStr::from("hello"), 1);
        pm.push(DictStr::from("he"), 2);
        pm.push(DictStr::from("hell"), 3);
        pm.push(DictStr::from("help"), 4);
        pm.push(DictStr::from("hey"), 5);
        pm.push(DictStr::from("hero"), 6);
        pm.push(DictStr::from("helvetica"), 7);
        pm.push(DictStr::from("potato"), 8);
        pm
    }

    fn value_for<T: Clone + Debug>(pm: &PrefixMap<T>, text: &str) -> T {
        let PrefixMapLookup::IsWord(word) = pm.lookup(&DictStr::from(text)) else {
            panic!("did not find {}", text)
        };
        (*word).clone()
    }

    #[test]
    fn longest_prefix_out_of_input_while_at_word() {
        let mut pm = build_test_prefix_map();
        assert_eq!(pm.longest_prefix("hell").unwrap(), value_for(&pm, "hell"));
    }

    #[test]
    fn longest_prefix_out_of_input_while_not_at_word() {
        let mut pm = build_test_prefix_map();
        assert_eq!(pm.longest_prefix("her").unwrap(), value_for(&pm, "he"));
    }

    #[test]
    fn longest_prefix_out_of_matches_while_deepest_is_word() {
        let mut pm = build_test_prefix_map();
        assert_eq!(pm.longest_prefix("helpless").unwrap(), value_for(&pm, "help"));
    }

    #[test]
    fn longest_prefix_out_of_matches_while_deepest_is_not_word() {
        let mut pm = build_test_prefix_map();
        assert_eq!(pm.longest_prefix("helve").unwrap(), value_for(&pm, "he"));
    }

    #[test]
    fn longest_prefix_unknown_prefix() {
        let mut pm = build_test_prefix_map();
        assert!(pm.longest_prefix("abacus").is_none());
    }

    #[test]
    fn test_all_prefixes_of_no_match() {
        let mut pm = build_test_prefix_map();
        assert!(pm.all_prefixes_of("abacus").is_empty());
    }

    #[test]
    fn test_all_prefixes_of_exact_match() {
        let mut pm = build_test_prefix_map();
        assert_eq!(pm.all_prefixes_of("hell"),
                   vec![value_for(&pm, "he"), value_for(&pm, &DictStr::from("hell"))]);
    }

    #[test]
    fn test_all_prefixes_of_sub_matches() {
        let mut pm = build_test_prefix_map();
        let mut pm = build_test_prefix_map();
        assert_eq!(pm.all_prefixes_of("helpless"),
                   vec![value_for(&pm, "he"), value_for(&pm, &DictStr::from("help"))]);
    }
}
