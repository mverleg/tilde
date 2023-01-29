
// This files uses len_utf8 for char length, based on the promise that str is utf8
// https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8

//  would it be faster to just store all strings in a hashmap and search all substrings one by one?
//  trie is more useful to e.g. find all things with a prefix, not all prefixes of a thing (although it works for that, just not faster)
//TODO @mverleg: remove this file once replaced

use ::std::collections::hash_map::Entry;
use ::std::collections::VecDeque;
use ::std::fmt::Debug;
use ::std::vec::IntoIter;

use crate::common::TinyMap;

type NodeIndex = u32;
const ROOT_INDEX: usize = 0;

#[derive(Debug)]
struct TrieNode<Word> {
    children: TinyMap<char, NodeIndex>,
    word: Option<Word>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrieLookup<'a, Word> {
    IsWord(&'a Word),
    IsPrefix,
    NotFound,
}

impl <Word: Debug> TrieNode<Word> {
    fn new_empty() -> Self {
        TrieNode {
            children: TinyMap::new(),
            word: None
        }
    }

    /// This takes `self_index`, which indexes into `nodes` to find current node. It does this
    /// 'hack' because the caller cannot borrow both this node, and the list of all nodes, mutably.
    /// But inside this method we can borrow all the nodes mutably one by one.
    ///
    /// (An alternative would be to only pass all subsequent nodes, not excluding the current one,
    /// which works because the child is always after the parent. But this would involve offset
    /// arithmetic, which adds complication and overhead).
    fn push(self_index: usize, text: &str, value: Word, nodes: &mut Vec<TrieNode<Word>>) {
        let new_child_index = nodes.len();
        let current = nodes.get_mut(self_index).expect("trie node missing");
        let head = match text.chars().next() {
            Some(chr) => chr,
            None => {
                current.word = Some(value);
                return
            },
        };
        let tail = &text[head.len_utf8()..];
        if let Some(child_index) = current.children.get(head) {
            Self::push(*child_index as usize, tail, value, nodes)
        } else {
            let mut child = TrieNode::new_empty();
            if tail.is_empty() {
                child.word = Some(value);
                nodes.push(child);
            } else {
                nodes.push(child);
                Self::push(new_child_index, tail, value, nodes);
            }
            let child_index = new_child_index.try_into().expect("INDX overflow, too many trie nodes");
            // re-borrow the current note here, to prevent lifetime conflicts
            nodes[self_index].children.insert(head, child_index);
        }
    }

    fn lookup<'a>(&'a self, text: &str, nodes: &'a [TrieNode<Word>]) -> TrieLookup<Word> {
        let head = match text.chars().next() {
            Some(chr) => chr,
            None => return match &self.word {
                Some(value) => TrieLookup::IsWord(value),
                None => TrieLookup::IsPrefix,
            },
        };
        let tail = &text[head.len_utf8()..];
        match self.children.get(head) {
            Some(child_index) => {
                let child = &nodes[*child_index as usize];
                child.lookup(tail, nodes)
            },
            None => TrieLookup::NotFound,
        }
    }

    fn all_prefixes_of<'a>(&'a self, text: &str, handler: &mut impl FnMut(&'a Word), nodes: &'a [TrieNode<Word>]) {
        if let Some(value) = &self.word {
            handler(value)
        }
        let Some(head) = text.chars().next() else {
            return;
        };
        let Some(next_index) = self.children.get(head) else {
            return;
        };
        let next = &nodes[*next_index as usize];
        let tail = &text[head.len_utf8()..];
        next.all_prefixes_of(tail, handler, nodes)
    }

    //TODO @mark:
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
    arena: Vec<TrieNode<Word>>,
}

impl <Word: Debug> Trie<Word> {
    pub fn new() -> Self {
        let root = TrieNode::new_empty();
        Trie {
            arena: vec![root],
        }
    }

    fn root(&self) -> &TrieNode<Word> {
        &self.arena[0]
    }

    fn root_mut(&mut self) -> &mut TrieNode<Word> {
        &mut self.arena[ROOT_INDEX]
    }

    pub fn push(&mut self, text: &str, value: Word) {
        TrieNode::push(ROOT_INDEX, text, value, &mut self.arena)
    }

    pub fn lookup(&self, value: &str) -> TrieLookup<Word> {
        self.root().lookup(value, &self.arena)
    }

    pub fn contains_exactly(&self, value: &str) -> bool {
        matches!(self.root().lookup(value, &self.arena), TrieLookup::IsWord(_))
    }

    /// Given a text, find all the words that are prefixes of it. E.g. "dogma" is ["do", "dog", "dogma"].
    pub fn all_prefixes_of(&self, text: &str) -> Vec<&Word> {
        let mut matches = Vec::new();
        self.all_prefixes_buffered_of(text, &mut matches);
        matches
    }

    /// Like `all_prefixes_of` but use existing buffer instead of allocating.
    pub fn all_prefixes_buffered_of<'a>(&'a self, text: &str, buffer: &mut Vec<&'a Word>) {
        buffer.clear();
        self.root().all_prefixes_of(text, &mut |word| buffer.push(word), &self.arena)
    }
}

impl <Word: Clone + Debug> Trie<Word> {

    pub fn longest_prefix(&self, text: &str) -> Option<Word> {
        let mut res = None;
        self.root().all_prefixes_of(text, &mut |word| res = Some(word), &self.arena);
        res.cloned()
    }

    pub fn all_prefixes_cloned_of(&self, text: &str, buffer: &mut Vec<Word>) {
        buffer.clear();
        self.root().all_prefixes_of(text, &mut |word| buffer.push((*word).clone()), &self.arena)
    }
}

#[cfg(test)]
mod tests {
    use ::std::mem::size_of;

    use crate::dict::INDX;

    use super::*;

    #[test]
    fn index_size() {
        assert!(size_of::<INDX>() <= size_of::<usize>(),
                "usize is smaller than index on this platform, this is not supported");
    }

    #[test]
    fn empty() {
        let trie = Trie::<()>::new();
        assert!(!trie.contains_exactly("hello"));
    }

    #[test]
    fn build() {
        let mut trie = Trie::new();
        trie.push("hello", 1);
        assert_eq!(trie.lookup("hello"), TrieLookup::IsWord(&1));
        assert_eq!(trie.lookup("he"), TrieLookup::IsPrefix);
        assert_eq!(trie.lookup("eh"), TrieLookup::NotFound);
        trie.push("he", 2);
        assert_eq!(trie.lookup("he"), TrieLookup::IsWord(&2));
        assert_eq!(trie.lookup("hel"), TrieLookup::IsPrefix);
        trie.push("hell", 3);
        assert_eq!(trie.lookup("hell"), TrieLookup::IsWord(&3));
        assert_eq!(trie.lookup("hel"), TrieLookup::IsPrefix);
        trie.push("hey", 4);
        assert_eq!(trie.lookup("hey"), TrieLookup::IsWord(&4));
        assert_eq!(trie.lookup("h"), TrieLookup::IsPrefix);
        assert_eq!(trie.lookup("p"), TrieLookup::NotFound);
        assert_eq!(trie.lookup("hello"), TrieLookup::IsWord(&1));
    }

    fn build_test_trie() -> Trie<i8> {
        let mut trie = Trie::new();
        trie.push("hello", 1);
        trie.push("he", 2);
        trie.push("hell", 3);
        trie.push("help", 4);
        trie.push("hey", 5);
        trie.push("hero", 6);
        trie.push("helvetica", 7);
        trie.push("potato", 8);
        trie
    }

    fn value_for<T: Clone + Debug>(trie: &Trie<T>, text: &str) -> T {
        let TrieLookup::IsWord(word) = trie.lookup(text) else {
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
        assert_eq!(trie.all_prefixes_of("hell"), vec![&value_for(&trie, "he"), &value_for(&trie, "hell")]);
    }

    #[test]
    fn test_all_prefixes_of_sub_matches() {
        let mut trie = build_test_trie();
        assert_eq!(trie.all_prefixes_of("helpless"), vec![&value_for(&trie, "he"), &value_for(&trie, "help")]);
    }
}
