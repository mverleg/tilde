use ::std::cmp::Ordering;
use ::std::fmt::Write;
use ::std::hash;
use ::std::hash::Hasher;

use ::tinyvec::ArrayVec;
use ::tinyvec_string::ArrayString;

use crate::common::dict_str::{CowDictStr, DictStrContent};
use crate::common::dict_str::DictStr;
use crate::common::dict_str::LONGEST_DICT_ENTRY_BYTES;
use crate::common::INDX;

pub type OpIndices = ArrayVec<[INDX; 4]>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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

    pub fn apply(&self, input: &'static str) -> DictStr {
        if self == &Self::new_noop() {
            return DictStr::from(input);
        }
        if input.len() <= self.pop_start as usize + self.pop_end as usize {
            return DictStr::from(input);
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
            return chars.into_iter()
                .collect::<DictStrContent>()
                .into()
        }
        // slice without alloc
        let mut end_index = input.len();
        for _ in 0..self.pop_end {
            let Some(chr) = chars.pop() else {
                return DictStr::from("");
            };
            end_index -= chr.len_utf8();
        }
        DictStr::from(&input[0..end_index])
    }

    pub fn operation_indices(&self) -> OpIndices {
        let mut indices = OpIndices::new();
        if self.case_first {
            indices.push(70);
        }
        if self.case_all {
            indices.push(71);
        }
        if self.reverse {
            indices.push(0);
            //TODO @mverleg:
        }
        for _ in 0..self.pop_start {
            indices.push(0);
            //TODO @mverleg:
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

    use crate::common::dict::DICT;
    use crate::common::dict::DictEntry;
    use crate::common::dict::iter_snippets;

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
}
