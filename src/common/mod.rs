pub use self::log::log;
pub use self::dict::lookup_buffer;
pub use self::dict::INDX;

mod dict;
mod log;
//mod compress;
//TODO @mark: ^
mod trie;
