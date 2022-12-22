pub use self::log::log;

mod log;
mod compress;
mod trie;

mod dict {
    //TODO @mark: move to separate file
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
    fn s(snip: &'static str) -> DictEntry {
        DictEntry::Snippet{ snip, capitalize_next: false, }
    }
    //noinspection RsFunctionNaming
    fn S(snip: &'static str) -> DictEntry {
        DictEntry::Snippet{ snip, capitalize_next: true, }
    }
    include!(concat!(env!("OUT_DIR"), "/dict_init.rs"));
}
