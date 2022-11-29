
//TODO @mark: $magic-newline-value$
//TODO @mark: 0 is reserved (for backspace)
//TODO @mark: fallback to full unicode after end?

use ::std::sync::{Arc, Mutex, RwLock};
use ::std::sync::LazyLock;
use ::std::slice::Iter;

static RAW_DICT: &'static str = include_str!("../../dictionary.txt");
static DICT: LazyLock<DictContainer> = LazyLock::new(|| DictContainer::new());

struct DictContainer {
    snippet_lookup: Vec<&'static str>,
}

#[derive(Clone)]
pub struct Dictionary {
}

impl DictContainer {
    fn new() -> Self {
        DictContainer {
            snippet_lookup: RAW_DICT.split("\n")
                .map(|line| if line != "$magic-newline-value$" { line } else { "\n" })
                .collect(),
        }
    }
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {}
    }

    pub fn index(&self, position: usize) -> Option<&'static str> {
        debug_assert!(position != 0, "cannot look up position 0, it is reserved (dict starts at 1)");
        DICT.snippet_lookup.get(position - 1).map(|txt| *txt)
    }

    fn iter(&self) -> Iter<'_, &'static str> {
        DICT.snippet_lookup.iter()
    }
}

#[cfg(test)]
mod lookup {
use ::std::collections::HashSet;
    use super::*;

    #[test]
    fn first_is_whitespace() {
        let dict = Dictionary::new();
        assert_eq!(dict.index(1), Some(" "), "first entry should be space (maybe stripped by editor?)");
    }

    #[test]
    fn trailing_whitespace() {
        let dict = Dictionary::new();
        let trailing_whitespace_count = dict.iter()
            .filter(|entry| entry.ends)
            .count();
        assert!(trailing_whitespace_count > 10, "quite some entries should have trailing space (maybe stripped by editor?)");
    }

    #[test]
    fn no_duplicates() {
        let dict = Dictionary::new();
        let mut seen = HashSet::new();
        for entry in dict.iter() {
            assert!(seen.insert(entry), "duplicate: {entry}");
        }
    }
}
