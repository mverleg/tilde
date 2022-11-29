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

    pub fn push(&mut self, value: &str) {
        let head = match value.chars().next() {
            Some(chr) => chr,
            None => return,
        };
        // based on the promise that str is utf8
        // https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8
        let tail = &value[head.len_utf8()..];
        match self.children.entry(head) {
            Entry::Occupied(mut child) => child.get_mut().push(tail),
            Entry::Vacant(mut entry) => {
                let mut child = TrieNode::new_empty();
                child.push(tail);
                entry.insert(child);
            }
        }
    }

    pub fn contains_exactly(&self, value: &str) -> bool {
        let head = match value.chars().next() {
            Some(chr) => chr,
            None => return self.is_word,
        };
        // based on the promise that str is utf8
        // https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8
        let tail = &value[head.len_utf8()..];
        return match self.children.get(&head) {
            Some(child) => child.contains_exactly(tail),
            None => false,
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
        self.root.push(value)
    }

    pub fn contains_exactly(&self, value: &str) -> bool {
        self.root.contains_exactly(value)
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
