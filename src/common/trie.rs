use ::std::collections::HashMap;

#[derive(Debug)]
enum TrieChildren {
    /// If there are only a few children, store them on the stack, with the empty ones at the end marked as dummy.
    Few([TrieNode; 3]),
    /// There are more than a few children, so we want to allocate a map for faster lookup
    Many(HashMap<char, TrieNode>),
}

#[derive(Debug, Default)]
enum TrieNodeType {
    /// Not an actual node, just a dummy in [TrieChildren::Few]
    #[default]
    Dummy,
    /// A node that is a word, and may also have children
    Word,
    /// A node that has children, but is not itself a word
    Link,
}

#[derive(Debug)]
struct TrieNode {
    typ: TrieNodeType,
    children: TrieChildren,
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: TrieNode { children: TrieChildren::empty(), typ: TrieNodeType::Dummy } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let trie = Trie::new();
    }
}
