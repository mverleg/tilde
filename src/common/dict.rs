
//TODO @mark: $magic-newline-value$
//TODO @mark: 0 is reserved (for backspace)
//TODO @mark: fallback to full unicode after end?

static RAW_DICT: &'static str = include_str!("../../dictionary.txt");
static DICT: Dictionary = Dictionary::new();

pub struct Dictionary {
    dict: Vec<&'static str>,
}

impl Dictionary {
    const fn new() -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod lookup {
    use super::*;

    #[test]
    fn test_whitespace() {
        assert!(DICT[0] == " ", "first entry should be space");
    }
}
