use ::std::borrow::Cow;

use crate::exec::Text;

#[derive(Debug, Clone, PartialEq)]
pub struct TextTransformation {
    pub case_first: bool,
    pub case_all: bool,
    pub pop_end: u8,
}

impl TextTransformation {
    pub fn new_noop() -> TextTransformation {
        TextTransformation {
            case_first: false,
            case_all: false,
            pop_end: 0,
        }
    }

    pub fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if self == &Self::new_noop() {
            return Cow::Borrowed(input)
        }
        todo!()
    }
}

fn switch_capitalization_char(orig_first: &mut char) {
    //TODO @mark: move this functions? add tests
    let mut upper = orig_first.to_uppercase();
    match upper.nth(0) {
        Some(switch_first) => {
            if switch_first != *orig_first {
                assert!(upper.nth(1).is_none(), "multi-char uppercase representations not yet supported");  //TODO @mark
                *orig_first = switch_first;
                return;
            }
        },
        None => {}
    };
    let mut lower = orig_first.to_lowercase();
    match lower.nth(0) {
        Some(switch_first) => {
            if switch_first != *orig_first {
                assert!(lower.nth(1).is_none(), "multi-char lowercase representations not yet supported");  //TODO @mark
                *orig_first = switch_first;
                return;
            }
        },
        None => {}
    }
}

#[cfg(test)]
mod capitalisation {
    use super::*;

    #[test]
    fn to_upper() {
        let mut letter = 'a';
        switch_capitalization_char(&mut letter);
        assert_eq!(letter, 'A');
    }

    #[test]
    fn to_lower() {
        let mut letter = 'A';
        switch_capitalization_char(&mut letter);
        assert_eq!(letter, 'a');
    }

    #[test]
    fn no_case() {
        let mut letter = '.';
        switch_capitalization_char(&mut letter);
        assert_eq!(letter, '.');
    }
}
