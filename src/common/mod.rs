pub use self::build_text::encode_snippet_len_estimate;
pub use self::dict::INDX;
pub use self::dict_lookup::lookup_buffer;
pub use self::log::log;
pub use self::text_trans::TextTransformation;

mod dict;
mod dict_lookup;
mod log;
//mod compress;
//TODO @mark: ^
mod trie;
mod text_trans;
mod build_text;
