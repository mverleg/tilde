
//TODO @mark: $magic-newline-value$
//TODO @mark: 0 is reserved (for backspace)
//TODO @mark: fallback to full unicode after end?

use ::std::sync::{Arc, Mutex, RwLock};

static RAW_DICT: &'static str = include_str!("../../dictionary.txt");
static DICT: Arc<RwLock<DictContainer>> = Arc::new(RwLock::new(DictContainer::new()));

struct DictContainer {
    snippet_lookup: Vec<&'static str>,
}

#[derive(Clone)]
pub struct Dictionary {
    container: Arc<RwLock<DictContainer>>,
}

impl DictContainer {
    const fn new() -> Self {
        DictContainer {
            snippet_lookup: vec![],
        }
    }
}

impl Dictionary {
    fn new() -> Self {
        Dictionary {
            container: DICT.clone(),
        }
    }

    fn index(&self, position: usize) -> Option<&'static str> {
        assert!(position != 0, "cannot look up position 0, it is reserved (dict starts at 1)");
        let read_container = self.container.read().expect("RwLock poisoned");
        if ! read_container.snippet_lookup.is_empty() {
            return read_container.snippet_lookup.get(position).map(|txt| *txt)
        }
        todo!()
    }
}

#[cfg(test)]
mod lookup {
    use super::*;

    #[test]
    fn test_whitespace() {
        let dict = Dictionary::new();
        assert_eq!(dict.index(0), Some(" "), "first entry should be space");
    }
}
