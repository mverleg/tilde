
// This files uses len_utf8 for char length, based on the promise that str is utf8
// https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8

//TODO @mark: iteration allocates a lot of strings, since each node only stores char
//TODO @mark: it would be possible to make iteration cheaper, if we'd store the whole text for each node

//TODO @mark: remove comments
//TODO @mark: enable tests

use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use ::std::collections::VecDeque;
use ::std::vec::IntoIter;
use crate::common::trie_original::TrieIterator;

#[derive(Debug)]
struct TrieNode<Word> {
    children: HashMap<char, TrieNode<Word>>,
    //TODO @mark: no alloc hashmap?
    word: Option<Word>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrieLookup {
    IsWord,
    IsPrefix,
    NotFound,
}

impl <Word> TrieNode<Word> {
    fn new_empty() -> Self {
        TrieNode {
            children: HashMap::with_capacity(0),
            word: None
        }
    }

    fn push(&mut self, value: impl AsRef<str>) {
        let text = value.as_ref();
        let head = match text.chars().next() {
            Some(chr) => chr,
            None => {
                self.word = true;
                return
            },
        };
        let tail = &text[head.len_utf8()..];
        match self.children.entry(head) {
            Entry::Occupied(mut child) => child.get_mut().push(tail),
            Entry::Vacant(mut entry) => {
                let mut child = TrieNode::new_empty();
                if tail.is_empty() {
                    child.word = true;
                } else {
                    child.push(tail);
                }
                entry.insert(child);
            }
        }
    }

    fn lookup(&self, value: &str) -> TrieLookup {
        let head = match value.chars().next() {
            Some(chr) => chr,
            None => return if self.word {
                TrieLookup::IsWord
            } else {
                TrieLookup::IsPrefix
            },
        };
        let tail = &value[head.len_utf8()..];
        return match self.children.get(&head) {
            Some(child) => child.lookup(tail),
            None => TrieLookup::NotFound,
        }
    }

    // fn contains_exactly(&self, value: &str) -> bool {
    //     self.lookup(value) == TrieLookup::IsWord
    // }
    //
    // fn longest_prefix(&self, value_remaining: &str, longest_word: &mut String, post_word: &mut String) {
    //     if self.word {
    //         longest_word.push_str(&post_word);
    //         post_word.clear();
    //     }
    //     let head = match value_remaining.chars().next() {
    //         Some(chr) => chr,
    //         None => {
    //             return
    //         },
    //     };
    //     let tail = &value_remaining[head.len_utf8()..];
    //     return match self.children.get(&head) {
    //         Some(child) => {
    //             post_word.push(head);
    //             child.longest_prefix(tail, longest_word, post_word)
    //         },
    //         None => return,
    //     }
    // }
    //
    // fn level_iterator_at_prefix(&self, initial_prefix: &str, remaining_value: &str) -> impl Iterator<Item = String> {
    //     let head = match remaining_value.chars().next() {
    //         Some(chr) => chr,
    //         None => {
    //             let mut child_texts = vec![];
    //             for child in &self.children {
    //                 let Some(word) = &child.1.word else {
    //                     continue
    //                 };
    //                 let mut text = initial_prefix.to_owned();
    //                 text.push(*child.0);
    //                 child_texts.push(text)
    //             }
    //             return child_texts.into_iter()
    //         },
    //     };
    //     let tail = &remaining_value[head.len_utf8()..];
    //     return match self.children.get(&head) {
    //         Some(child) => child.level_iterator_at_prefix(initial_prefix, tail),
    //         None => vec![].into_iter(),
    //     }
    // }
}

#[derive(Debug)]
pub struct Trie<Word> {
    root: TrieNode<Word>,
}

impl <Word> Trie<Word> {
    pub fn new() -> Self {
        Trie {
            root:TrieNode::new_empty(),
        }
    }

    pub fn push(&mut self, value: impl AsRef<str>) {
        self.root.push(value)
    }

    pub fn lookup(&self, value: &str) -> TrieLookup {
        self.root.lookup(value)
    }

    pub fn contains_exactly(&self, value: &str) -> bool {
        self.root.lookup(value) == TrieLookup::IsWord
    }

    /// Given a text, find all the words that are prefixes of it. E.g. "dogma" is ["do", "dog", "dogma"].
    pub fn all_prefixes_of(&self, value: &str) {
        todo!()  //TODO @mark:
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn empty() {
//         let trie = Trie::new();
//         assert!(!trie.contains_exactly("hello"));
//     }
//
//     #[test]
//     fn build() {
//         let mut trie = Trie::new();
//         trie.push("hello");
//         assert_eq!(trie.lookup("hello"), TrieLookup::IsWord);
//         assert_eq!(trie.lookup("he"), TrieLookup::IsPrefix);
//         assert_eq!(trie.lookup("eh"), TrieLookup::NotFound);
//         trie.push("he");
//         assert_eq!(trie.lookup("he"), TrieLookup::IsWord);
//         assert_eq!(trie.lookup("hel"), TrieLookup::IsPrefix);
//         trie.push("hell");
//         assert_eq!(trie.lookup("hell"), TrieLookup::IsWord);
//         assert_eq!(trie.lookup("hel"), TrieLookup::IsPrefix);
//         trie.push("hey");
//         assert_eq!(trie.lookup("hey"), TrieLookup::IsWord);
//         assert_eq!(trie.lookup("h"), TrieLookup::IsPrefix);
//         assert_eq!(trie.lookup("p"), TrieLookup::NotFound);
//     }
//
//     fn build_test_trie() -> Trie {
//         let mut trie = Trie::new();
//         trie.push("hello");
//         trie.push("he");
//         trie.push("hell");
//         trie.push("help");
//         trie.push("hey");
//         trie.push("hero");
//         trie.push("helvetica");
//         trie.push("potato");
//         trie
//     }
//
//     #[test]
//     fn prefix_iter_deep() {
//         let trie = build_test_trie();
//         let mut matches = trie.iter_prefix("hel")
//             .collect::<Vec<_>>();
//         matches.sort();
//         assert_eq!(matches, vec!["hell", "hello", "help", "helvetica"]);
//     }
//
//     #[test]
//     fn prefix_iter_shallow() {
//         let mut trie = build_test_trie();
//         let mut matches = trie.iter_one_extra_letter("hel")
//             .collect::<Vec<_>>();
//         matches.sort();
//         assert_eq!(matches, vec!["hell", "help"]);
//     }
//
//     #[test]
//     fn longest_prefix_out_of_input_while_at_word() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.longest_prefix("hell"), "hell");
//     }
//
//     #[test]
//     fn longest_prefix_out_of_input_while_not_at_word() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.longest_prefix("her"), "he");
//     }
//
//     #[test]
//     fn longest_prefix_out_of_matches_while_deepest_is_word() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.longest_prefix("helpless"), "help");
//     }
//
//     #[test]
//     fn longest_prefix_out_of_matches_while_deepest_is_not_word() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.longest_prefix("helve"), "he");
//     }
//
//     #[test]
//     fn longest_prefix_unknown_prefix() {
//         let mut trie = build_test_trie();
//         assert_eq!(trie.longest_prefix("abacus"), "");
//     }
//
//     #[test]
//     fn longest_prefix_with_buffer() {
//         let mut result_buffer = "clear this".to_owned();
//         let mut postfix_buffer = "clear this".to_owned();
//         let mut trie = build_test_trie();
//         trie.longest_prefix_with("her", &mut result_buffer, &mut postfix_buffer);
//         assert_eq!(result_buffer, "he");
//         assert_eq!(postfix_buffer, "r");
//     }
// }
