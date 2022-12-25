/// Dictionary entry transformations.
/// This file is also included from `build.rs`

use ::std::borrow::Cow;
use ::std::fmt::Write;

use ::tinyvec::ArrayVec;

#[allow(dead_code)]
pub const LONGEST_DICT_ENTRY_BYTES: usize = 22;  // located in this file because of build.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextTransformation {
    pub case_first: bool,
    pub case_all: bool,
    pub reverse: bool,
    pub pop_start: u8,
    pub pop_end: u8,
}

impl TextTransformation {
    pub fn new_noop() -> TextTransformation {
        TextTransformation {
            case_first: false,
            case_all: false,
            reverse: false,
            pop_start: 0,
            pop_end: 0,
        }
    }

    pub fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if self == &Self::new_noop() {
            return Cow::Borrowed(input);
        }
        if input.len() <= self.pop_start as usize + self.pop_end as usize {
            return Cow::Borrowed(input);
        }
        assert!(self.pop_start == 0, "pop_start not impl");
        assert!(!self.reverse, "reverse not impl");
        let mut chars = input.chars().collect::<ArrayVec<[char; LONGEST_DICT_ENTRY_BYTES]>>();
        if self.case_all || self.case_first {
            // need to alloc string
            for _ in 0..self.pop_end {
                chars.pop();
            }
            assert!(!self.case_all, "case_all not impl");
            if self.case_first {
                switch_capitalization_char(&mut chars[0])
            }
            return Cow::Owned(chars.into_iter().collect::<String>())
        }
        // slice without alloc
        let mut end_index = input.len();
        for _ in 0..self.pop_end {
            let Some(chr) = chars.pop() else {
                return Cow::Borrowed("");
            };
            end_index -= chr.len_utf8();
        }
        Cow::Borrowed(&input[0..end_index])
    }

    pub fn name(&self) -> String {
        let mut repr = String::with_capacity(6);
        write!(repr, "{}", match (self.case_all, self.case_first) {
            (true, false) => 'a',
            (false, true) => 'f',
            (true, true) => 'w',
            (false, false) => 'n',
        }).unwrap();
        write!(repr, "{}", if self.reverse { 'r' } else { 'i' }).unwrap();
        if self.pop_start >= 10 || self.pop_end >= 10 {
            write!(repr, "{}_{}", self.pop_start, self.pop_end).unwrap();
        } else {
            write!(repr, "{}{}", self.pop_start, self.pop_end).unwrap();
        }
        repr
    }
}

#[allow(dead_code)]
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
        }
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
        }
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
