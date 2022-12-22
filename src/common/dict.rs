#![allow(warnings)]
#![allow(clippy::all)]

pub type INDX = u16;

#[derive(Debug, Clone, Copy)]
pub enum DictEntry {
    Snippet { snip: &'static str, capitalize_next: bool, },
    Backspace,
    CapitalizeFirst,
    CapitalizeAll,
}

const fn s(snip: &'static str) -> DictEntry {
    DictEntry::Snippet{ snip, capitalize_next: false, }
}
//noinspection RsFunctionNaming
const fn S(snip: &'static str) -> DictEntry {
    DictEntry::Snippet{ snip, capitalize_next: true, }
}

include!(concat!(env!("OUT_DIR"), "/dict_init.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dict_index_width() {
        assert!((DICT.len() as u64) < (INDX::MAX as u64));
    }
}
