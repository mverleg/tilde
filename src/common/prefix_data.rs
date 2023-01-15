
// This files uses len_utf8 for char length, based on the promise that str is utf8
// https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8

//TODO @mark: remove comments

//TODO @mverleg: would it be faster to just store all strings in a hashmap and search all substrings one by one?
//  trie is more useful to e.g. find all things with a prefix, not all prefixes of a thing (although it works for that, just not faster)

use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use ::std::collections::VecDeque;
use ::std::fmt::Debug;
use ::std::vec::IntoIter;

use crate::common::INDX;
use crate::common::text_trans::{CowDictStr, DictStr, LONGEST_DICT_ENTRY_BYTES};
use crate::common::tiny_map::TinyMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrieLookup<'a, Word> {
    IsWord(&'a Word),
    IsPrefix,
    NotFound,
}

#[derive(Debug)]
pub struct PrefixMap<Word> {
    words: HashMap<DictStr, Word>,
}

impl <Word: Debug> PrefixMap<Word> {
    pub fn new() -> Self {
        PrefixMap {
            words: HashMap::new(),
        }
    }

    pub fn push(&mut self, text: DictStr, value: Word) {
        debug_assert!(!self.words.contains_key(&text));
        self.words.insert(text, value);
    }

    pub fn contains_exactly(&self, value: &DictStr) -> bool {
        self.words.get(value).is_some()
    }
}

impl <Word: Clone + Debug> PrefixMap<Word> {

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

//TODO @mverleg: enable tests

// #[cfg(test)]
// mod tests {
//     use ::std::mem::size_of;
//
//     use super::*;
//
//     #[test]
//     fn index_size() {
//         assert!(size_of::<INDX>() <= size_of::<usize>(),
//                 "usize is smaller than index on this platform, this is not supported");
//     }
//
//     #[test]
//     fn empty() {
//         let trie = PrefixMap::<()>::new();
//         assert!(!trie.contains_exactly("hello"));
//     }
//
//     #[test]
//     fn build() {
//         let mut trie = PrefixMap::new();
//         trie.push("hello", 1);
//         assert_eq!(trie.lookup("hello"), TrieLookup::IsWord(&1));
//         assert_eq!(trie.lookup("he"), TrieLookup::IsPrefix);
//         assert_eq!(trie.lookup("eh"), TrieLookup::NotFound);
//         trie.push("he", 2);
//         assert_eq!(trie.lookup("he"), TrieLookup::IsWord(&2));
//         assert_eq!(trie.lookup("hel"), TrieLookup::IsPrefix);
//         trie.push("hell", 3);
//         assert_eq!(trie.lookup("hell"), TrieLookup::IsWord(&3));
//         assert_eq!(trie.lookup("hel"), TrieLookup::IsPrefix);
//         trie.push("hey", 4);
//         assert_eq!(trie.lookup("hey"), TrieLookup::IsWord(&4));
//         assert_eq!(trie.lookup("h"), TrieLookup::IsPrefix);
//         assert_eq!(trie.lookup("p"), TrieLookup::NotFound);
//         assert_eq!(trie.lookup("hello"), TrieLookup::IsWord(&1));
//     }
//
//     fn build_test_trie() -> PrefixMap<i8> {
//         let mut trie = PrefixMap::new();
//         trie.push("hello", 1);
//         trie.push("he", 2);
//         trie.push("hell", 3);
//         trie.push("help", 4);
//         trie.push("hey", 5);
//         trie.push("hero", 6);
//         trie.push("helvetica", 7);
//         trie.push("potato", 8);
//         trie
//     }
//
//     fn value_for<T: Clone + Debug>(trie: &PrefixMap<T>, text: &str) -> T {
//         let TrieLookup::IsWord(word) = trie.lookup(text) else {
//             panic!("did not find {}", text)
//         };
//         (*word).clone()
//     }
//
//     #[test]
//     fn longest_prefix_out_of_input_while_at_word() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.longest_prefix("hell").unwrap(), value_for(&trie, "hell"));
//     }
//
//     #[test]
//     fn longest_prefix_out_of_input_while_not_at_word() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.longest_prefix("her").unwrap(), value_for(&trie, "he"));
//     }
//
//     #[test]
//     fn longest_prefix_out_of_matches_while_deepest_is_word() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.longest_prefix("helpless").unwrap(), value_for(&trie, "help"));
//     }
//
//     #[test]
//     fn longest_prefix_out_of_matches_while_deepest_is_not_word() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.longest_prefix("helve").unwrap(), value_for(&trie, "he"));
//     }
//
//     #[test]
//     fn longest_prefix_unknown_prefix() {
//         let mut trie = build_test_trie();
//         assert!(trie.longest_prefix("abacus").is_none());
//     }
//
//     #[test]
//     fn test_all_prefixes_of_no_match() {
//         let mut trie = build_test_trie();
//         assert!(trie.all_prefixes_of("abacus").is_empty());
//     }
//
//     #[test]
//     fn test_all_prefixes_of_exact_match() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.all_prefixes_of("hell"), vec![&value_for(&trie, "he"), &value_for(&trie, "hell")]);
//     }
//
//     #[test]
//     fn test_all_prefixes_of_sub_matches() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.all_prefixes_of("helpless"), vec![&value_for(&trie, "he"), &value_for(&trie, "help")]);
//     }
// }
