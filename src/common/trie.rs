
// This files uses len_utf8 for char length, based on the promise that str is utf8
// https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8

//TODO @mark: iteration allocates a lot of strings, since each node only stores char
//TODO @mark: it would be possible to make iteration cheaper, if we'd store the whole text for each node

use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use ::std::collections::VecDeque;

//TODO: maybe make this a separate crate (but there are already 2 - one has too many dependencies for my taste, and the other seems dead)

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrieLookup {
    IsWord,
    IsPrefix,
    NotFound,
}

impl TrieNode {
    pub fn new_empty() -> Self {
        TrieNode {
            children: HashMap::with_capacity(0),
            is_word: false
        }
    }

    pub fn push(&mut self, value: &str) {
        let head = match value.chars().next() {
            Some(chr) => chr,
            None => {
                self.is_word = true;
                return
            },
        };
        let tail = &value[head.len_utf8()..];
        match self.children.entry(head) {
            Entry::Occupied(mut child) => child.get_mut().push(tail),
            Entry::Vacant(mut entry) => {
                let mut child = TrieNode::new_empty();
                if tail.is_empty() {
                    child.is_word = true;
                } else {
                    child.push(tail);
                }
                entry.insert(child);
            }
        }
    }

    pub fn lookup(&self, value: &str) -> TrieLookup {
        let head = match value.chars().next() {
            Some(chr) => chr,
            None => return if self.is_word {
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

    pub fn contains_exactly(&self, value: &str) -> bool {
        self.lookup(value) == TrieLookup::IsWord
    }

    pub fn iterator_at_prefix(&self, initial_prefix: &str, remaining_value: &str) -> TrieIterator {
        eprintln!("iterator_at_prefix: {}", initial_prefix);  //TODO @mark: TEMPORARY! REMOVE THIS!
        let head = match remaining_value.chars().next() {
            Some(chr) => chr,
            None => return TrieIterator::new_at(initial_prefix.to_owned(), self),
        };
        let tail = &remaining_value[head.len_utf8()..];
        return match self.children.get(&head) {
            Some(child) => child.iterator_at_prefix(initial_prefix, tail),
            None => TrieIterator::new_empty(),
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode
}

#[derive(Debug)]
struct TrieNodePrefix<'a> {
    prefix: String,
    node: &'a TrieNode,
}

impl <'a> TrieNodePrefix<'a> {
    pub fn new(prefix: String, node: &'a TrieNode) -> Self {
        TrieNodePrefix { prefix, node }
    }
}

// Breadth-first iterator, ordering of elements is undefined (depends on hashes).
#[derive(Debug)]
pub struct TrieIterator<'a> {
    nodes: VecDeque<TrieNodePrefix<'a>>,
}

impl <'a> TrieIterator<'a> {
    fn new_at(prefix: String, elem: &'a TrieNode) -> Self {
        let mut nodes = VecDeque::new();
        eprintln!("pushing initial: {}", &prefix);  //TODO @mark: TEMPORARY! REMOVE THIS!
        nodes.push_back(TrieNodePrefix::new(prefix, elem));
        TrieIterator {
            nodes,
        }
    }
    
    fn new_empty() -> Self {
        TrieIterator { nodes: VecDeque::new() }
    }
}

impl <'a> Iterator for TrieIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(elem) = self.nodes.pop_front() {
            for child in &elem.node.children {
                let mut text = elem.prefix.to_owned();
                eprintln!("pushing child: {} + {}", &text, child.0);  //TODO @mark: TEMPORARY! REMOVE THIS!
                text.push(*child.0);
                self.nodes.push_back(TrieNodePrefix::new(text, child.1))
            }
            if elem.node.is_word {
                return Some(elem.prefix)
            }
        }
        None
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root:TrieNode::new_empty()
        }
    }

    pub fn push(&mut self, value: &str) {
        self.root.push(value)
    }

    pub fn lookup(&self, value: &str) -> TrieLookup {
        self.root.lookup(value)
    }

    pub fn contains_exactly(&self, value: &str) -> bool {
        self.root.lookup(value) == TrieLookup::IsWord
    }

    pub fn iter_prefix(&self, prefix: &str) -> TrieIterator {
        self.root.iterator_at_prefix(prefix, prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let trie = Trie::new();
        assert!(!trie.contains_exactly("hello"));
    }

    #[test]
    fn build() {
        let mut trie = Trie::new();
        trie.push("hello");
        assert_eq!(trie.lookup("hello"), TrieLookup::IsWord);
        assert_eq!(trie.lookup("he"), TrieLookup::IsPrefix);
        assert_eq!(trie.lookup("eh"), TrieLookup::NotFound);
        trie.push("he");
        assert_eq!(trie.lookup("he"), TrieLookup::IsWord);
        assert_eq!(trie.lookup("hel"), TrieLookup::IsPrefix);
        trie.push("hell");
        assert_eq!(trie.lookup("hell"), TrieLookup::IsWord);
        assert_eq!(trie.lookup("hel"), TrieLookup::IsPrefix);
        trie.push("hey");
        assert_eq!(trie.lookup("hey"), TrieLookup::IsWord);
        assert_eq!(trie.lookup("h"), TrieLookup::IsPrefix);
        assert_eq!(trie.lookup("p"), TrieLookup::NotFound);
    }

    #[test]
    fn prefix_iter() {
        let mut trie = Trie::new();
        trie.push("hello");
        trie.push("he");
        trie.push("hell");
        trie.push("help");
        trie.push("hey");
        trie.push("potato");
        let mut matches = trie.iter_prefix("hel")
            .collect::<Vec<_>>();
        matches.sort();
        assert_eq!(matches, vec!["hell", "hello", "help"]);
    }
}
