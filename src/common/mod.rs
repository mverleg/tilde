pub use self::text_trans::TextTransformation;
pub use self::dict::INDX;
pub use self::dict::lookup_buffer;
pub use self::log::log;

mod dict;
mod log;
//mod compress;
//TODO @mark: ^
mod trie;
mod text_trans;
