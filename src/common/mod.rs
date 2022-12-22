pub use self::log::log;

mod log;
mod compress;
mod trie;

mod dict {
    include!(concat!(env!("OUT_DIR"), "/dict_init.rs"));
}
