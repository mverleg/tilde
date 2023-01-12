use ::std::cmp::Ordering;
use ::std::fmt::Write;
use ::std::hash;
use ::std::hash::Hasher;
use ::tinyvec::ArrayVec;
use ::tinyvec_string::ArrayString;

use crate::common::INDX;

#[allow(dead_code)]
pub const LONGEST_DICT_ENTRY_BYTES: usize = 22; // located in this file because of build.rs
pub type DictStr = ArrayString<[u8; LONGEST_DICT_ENTRY_BYTES]>;
pub type OpIndices = ArrayVec<[INDX; 4]>;
#[derive(Debug, Eq, Ord)]
pub enum CowDictStr {
    Owned(DictStr),
    Borrowed(&'static str),
}

impl CowDictStr {
    pub fn into_owned(self) -> DictStr {
        match self {
            CowDictStr::Owned(val) => val,
            CowDictStr::Borrowed(val) => DictStr::from(val),
        }
    }
}

impl AsRef<str> for CowDictStr {
    fn as_ref(&self) -> &str {
        match self {
            CowDictStr::Owned(text) => &*text,
            CowDictStr::Borrowed(text) => *text,
        }
    }
}

impl PartialEq for CowDictStr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CowDictStr::Owned(left), CowDictStr::Owned(right)) => left.as_str() == right.as_str(),
            (CowDictStr::Borrowed(left), CowDictStr::Owned(right)) => *left == right.as_str(),
            (CowDictStr::Owned(left), CowDictStr::Borrowed(right)) => left.as_str() == *right,
            (CowDictStr::Borrowed(left), CowDictStr::Borrowed(right)) => left == right,
        }
    }
}

impl PartialOrd for CowDictStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (CowDictStr::Owned(left), CowDictStr::Owned(right)) => left.as_str().partial_cmp(right.as_str()),
            (CowDictStr::Borrowed(left), CowDictStr::Owned(right)) => left.partial_cmp(&right.as_str()),
            (CowDictStr::Owned(left), CowDictStr::Borrowed(right)) => left.as_str().partial_cmp(right),
            (CowDictStr::Borrowed(left), CowDictStr::Borrowed(right)) => left.partial_cmp(right),
        }
    }
}

impl hash::Hash for CowDictStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.as_ref().as_bytes())
    }
}

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

    pub fn apply(&self, input: &'static str) -> CowDictStr {
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
            return CowDictStr::Owned(chars.into_iter().collect::<DictStr>())
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
            indices.push(0);
            //TODO @mverleg:
        }
        if self.case_all {
            indices.push(0);
            //TODO @mverleg:
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
            //TODO @mverleg:
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

    use super::*;
    use crate::common::dict::iter_snippets;
    use crate::common::dict::DictEntry;
    use crate::common::dict::DICT;

    thread_local! {
        static DICT_POSITIONS: LazyCell<HashMap<DictEntry, usize>> = LazyCell::new(||
            DICT.iter().enumerate()
                .map(|(index, entry)| (*entry, index))
                .collect());
    }

    #[test]
    fn case_first() {
        let trans = TextTransformation { case_first: true, ..Default::default() };
        let indices = trans.operation_indices();
        assert!(indices.len() == 1);
        let index: usize = indices[0]
            .try_into()
            .expect("could not convert index to usize");
        let expected = DICT_POSITIONS.with(|dict_pos| {
            *dict_pos
                .get(&DictEntry::CapitalizeFirst)
                .unwrap()
        });
        assert_eq!(index, expected);
    }
}
