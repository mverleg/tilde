use ::std::collections::HashMap;
use ::std::collections::hash_map::Entry;

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    pub fn new_empty() -> Self {
        TrieNode {
            children: HashMap::with_capacity(0),
            is_word: false
        }
    }

    pub fn push(&mut self, chars: &[char]) {
        let (head, tail) = match chars.split_first() {
            Some(split) => split,
            None => {
                eprintln!("reached end!");  //TODO @mark: TEMPORARY! REMOVE THIS!
                return
            },
        };
        match self.children.entry(*head) {
            Entry::Occupied(mut child) => child.get_mut().push(tail),
            Entry::Vacant(mut entry) => {
                let mut child = TrieNode::new_empty();
                child.push(tail);
                entry.insert(child);
            }
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root:TrieNode::new_empty()
        }
    }

    pub fn push(&mut self, value: &str) {
        let chars = value.chars().collect::<Vec<_>>();
        self.root.push(&chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let trie = Trie::new();
        assert!(trie.contains_exactly("hello"));
    }

    #[test]
    fn build() {
        let mut trie = Trie::new();
        trie.push("hello");
        assert!(trie.contains_exactly("hello"));
        assert!(!trie.contains_exactly("he"));
        trie.push("he");
        assert!(trie.contains_exactly("he"));
        assert!(!trie.contains_exactly("hel"));
        trie.push("hell");
        assert!(trie.contains_exactly("hell"));
        assert!(!trie.contains_exactly("hel"));
        trie.push("hey");
        assert!(trie.contains_exactly("hey"));
        assert!(!trie.contains_exactly("h"));
        assert!(!trie.contains_exactly("p"));
    }
}
