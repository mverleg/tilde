use ::std::collections::HashMap;

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
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let trie = Trie::new();
        //TODO @mark: lookup
    }

    #[test]
    fn build() {
        let mut trie = Trie::new();
        trie.push("hello");
        trie.push("he");
        trie.push("hell");
        trie.push("hey");
    }
}
