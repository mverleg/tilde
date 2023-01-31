use ::std::cmp::Ordering;
use ::std::fmt::Write;
use ::std::hash;
use ::std::hash::Hasher;

use ::tinyvec::ArrayVec;
use ::tinyvec_string::ArrayString;

use crate::dict::{CowDictStr, DictStr, DictStrContent, INDX, LONGEST_DICT_ENTRY_BYTES, MAX_TEXT_TRANSFORMS};
use crate::tilde_log;

pub type OpIndices = ArrayVec<[INDX; MAX_TEXT_TRANSFORMS]>;

pub const UNICODE_MAGIC_INDX: INDX = 70;
//TODO @mark: move all derived data to one module, or generate from build.rs

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextTransformation {
    pub case_first: bool,
    pub case_all: bool,
    pub reverse: bool,
    pub pop_start: u8,
    pub pop_end: u8,
}

#[derive(Debug)]
pub enum SnipOrChar {
    Snip(&'static str),
    Char(char),
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

    pub fn apply(&self, input: SnipOrChar) -> CowDictStr {
        if self != &Self::new_noop() {  // should get optimized away in release mode
            tilde_log!("transform: case_first={} case_all={} reverse={} pop_start={} pop_end={}",
                self.case_first, self.case_all, self.reverse, self.pop_start, self.pop_end);
        }
        match input {
            SnipOrChar::Snip(text) => self.apply_str(text),
            SnipOrChar::Char(letter) => CowDictStr::Owned(self.apply_char(letter)
                .map_or_else(|| DictStr::empty(), |c| DictStr::from_char(c))),
        }
    }

    fn apply_char(&self, input: char) -> Option<char> {
        if self.pop_start > 0 || self.pop_end > 0 {
            return None;
        }
        let mut letter = input;
        if self.case_all || self.case_first {
            switch_capitalization_char(&mut letter);
        }
        return Some(letter)
    }

    pub fn apply_str(&self, input: &'static str) -> CowDictStr {
        if self == &Self::new_noop() {
            return CowDictStr::Borrowed(input);
        }
        if input.len() <= self.pop_start as usize + self.pop_end as usize {
            return CowDictStr::Borrowed(input);
        }
        assert!(self.pop_start == 0, "pop_start not impl");
        assert!(!self.reverse, "reverse not impl");
        let mut chars = input.chars().collect::<ArrayVec<[char; LONGEST_DICT_ENTRY_BYTES]>>();
        if self.case_all || self.case_first {
            // need to create string
            for _ in 0..self.pop_end {
                chars.pop();
            }
            assert!(!self.case_all, "case_all not impl");
            if self.case_first {
                switch_capitalization_char(&mut chars[0])
            }
            return CowDictStr::Owned(chars.into_iter()
                .collect::<DictStrContent>()
                .into())
        }
        // slice without alloc
        let mut end_index = input.len();
        for _ in 0..self.pop_end {
            let Some(chr) = chars.pop() else {
                return CowDictStr::Borrowed("");
            };
            end_index -= chr.len_utf8();
        }
        CowDictStr::Borrowed(&input[0..end_index])
    }

    pub fn operation_indices(&self) -> OpIndices {
        let mut indices = OpIndices::new();
        if self.case_first {
            indices.push(71);
        }
        if self.case_all {
            indices.push(72);
        }
        if self.reverse {
            indices.push(304);
        }
        for _ in 0..self.pop_start {
            indices.push(303);
        }
        for _ in 0..self.pop_end {
            indices.push(0);
        }
        indices
    }

    pub fn name(&self) -> ArrayString<[u8; 6]> {
        let mut repr = ArrayString::new();
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
        ArrayString::try_from(repr).expect("name is too long for array string")
    }
}

#[allow(dead_code)]
fn switch_capitalization_char(orig_first: &mut char) {
    //TODO @mark: move this functions? add tests
    let mut upper = orig_first.to_uppercase();
    match upper.nth(0) {
        Some(switch_first) => {
            if switch_first != *orig_first {
                assert!(upper.nth(1).is_none(), "multi-char uppercase representations not yet supported"); //TODO @mark
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
                assert!(lower.nth(1).is_none(), "multi-char lowercase representations not yet supported"); //TODO @mark
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
        let mut letter = 'é';
        switch_capitalization_char(&mut letter);
        assert_eq!(letter, 'É');
    }

    #[test]
    fn to_lower() {
        let mut letter = 'A';
        switch_capitalization_char(&mut letter);
        assert_eq!(letter, 'a');
        let mut letter = 'É';
        switch_capitalization_char(&mut letter);
        assert_eq!(letter, 'é');
    }

    #[test]
    fn no_case() {
        let mut letter = '.';
        switch_capitalization_char(&mut letter);
        assert_eq!(letter, '.');
        let mut letter = '🦀';
        switch_capitalization_char(&mut letter);
        assert_eq!(letter, '🦀');
    }
}

#[cfg(test)]
mod indices_in_sync_with_dict {
    use ::std::cell::LazyCell;
    use ::std::collections::HashMap;

    use ::strum::IntoEnumIterator;

    use crate::dict::DICT;
    use crate::dict::DictEntry;

    use super::*;

    thread_local! {
        static DICT_POSITIONS: LazyCell<HashMap<DictEntry, usize>> = LazyCell::new(||
            DICT.iter().enumerate()
                .map(|(index, entry)| (*entry, index))
                .collect());
    }

    fn assert_transformation_index(trans: TextTransformation, entry: DictEntry) {
        let indices = trans.operation_indices();
        assert!(indices.len() == 1);
        let index: usize = indices[0]
            .try_into()
            .expect("could not convert index to usize");
        let expected = DICT_POSITIONS.with(|dict_pos| {
            *dict_pos
                .get(&entry)
                .unwrap()
        });
        assert_eq!(index, expected, "index should be {expected} but is {index}");
    }

    #[test]
    fn case_first() {
        assert_transformation_index(
            TextTransformation { case_first: true, ..Default::default() },
            DictEntry::CapitalizeFirst)
    }

    #[test]
    fn case_all() {
        assert_transformation_index(
            TextTransformation { case_all: true, ..Default::default() },
            DictEntry::CapitalizeAll)
    }

    #[test]
    fn pop_end() {
        assert_transformation_index(
            TextTransformation { pop_end: 1, ..Default::default() },
            DictEntry::Backspace)
    }

    #[test]
    fn pop_start() {
        assert_transformation_index(
            TextTransformation { pop_start: 1, ..Default::default() },
            DictEntry::BackspaceFront)
    }

    #[test]
    fn reverse() {
        assert_transformation_index(
            TextTransformation { reverse: true, ..Default::default() },
            DictEntry::Reverse)
    }

    #[test]
    fn unicode_lookup() {
        let entry = DictEntry::UnicodeLookup;
        let index: usize = UNICODE_MAGIC_INDX
            .try_into()
            .expect("could not convert index to usize");
        let expected = DICT_POSITIONS.with(|dict_pos| {
            *dict_pos
                .get(&entry)
                .unwrap()
        });
        assert_eq!(index, expected, "index should be {expected} but is {index}");
    }
}
