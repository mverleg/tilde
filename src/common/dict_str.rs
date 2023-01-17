use ::std::cmp::Ordering;
use ::std::fmt::Write;
use ::std::hash;
use ::std::hash::Hasher;

use ::fnv::FnvHasher;
use ::tinyvec::ArrayVec;
use ::tinyvec_string::ArrayString;

use crate::common::INDX;

#[allow(dead_code)]
pub const LONGEST_DICT_ENTRY_BYTES: usize = 22; // located in this file because of build.rs
pub type DictStrContent = ArrayString<[u8; LONGEST_DICT_ENTRY_BYTES]>;

#[derive(Debug)]
pub struct DictStr {
    text: DictStrContent,
    cached_hash: u32,
}

impl DictStr {
    pub fn new(text: DictStrContent) -> Self {
        let hash = {
            let mut hasher = FnvHasher::default();
            hasher.write(text.as_bytes());
            hasher.finish() as u32
        };
        DictStr {
            text,
            cached_hash: hash,
        }
    }

    pub fn from(text: impl AsRef<str>) -> Self {
        DictStr::new(DictStrContent::from(text.as_ref()))
    }

    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }
}

impl PartialEq for DictStr {
    fn eq(&self, other: &Self) -> bool {
        self.text.eq(&other.text)
    }
}

impl Eq for DictStr {}

impl hash::Hash for DictStr {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.cached_hash)
    }
}

impl PartialOrd for DictStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.text.partial_cmp(&other.text)
    }
}

impl Ord for DictStr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.text.cmp(&other.text)
    }
}

impl From<DictStrContent> for DictStr {
    fn from(value: DictStrContent) -> Self {
        DictStr::new(value)
    }
}

impl ToOwned for DictStr {
    type Owned = DictStr;

    fn to_owned(&self) -> Self::Owned {
        DictStr {
            text: self.text.clone(),
            cached_hash: self.cached_hash,
        }
    }
}

#[derive(Debug, Eq, Ord)]
pub enum CowDictStr {
    Owned(DictStr),
    Borrowed(&'static str),
}

impl CowDictStr {
    pub fn to_owned(&self) -> DictStr {
        match self {
            CowDictStr::Owned(val) => val.to_owned(),
            CowDictStr::Borrowed(val) => DictStr::from(val),
        }
    }
}

impl AsRef<str> for CowDictStr {
    fn as_ref(&self) -> &str {
        match self {
            CowDictStr::Owned(text) => text.text.as_str(),
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
