pub use self::dict::DICT;
pub use self::dict::INDX;
pub use self::dict_compress::compress_with_dict;
pub use self::dict_lookup::lookup_buffer;
pub use self::escape_str::escape_for_string;
pub use self::escape_str::is_safe_for_string;
pub use self::log::log;
pub use self::text_trans::SnipOrChar;
pub use self::text_trans::TextTransformation;
pub use self::text_trans::UNICODE_MAGIC_INDX;
pub use self::tiny_map::TinyMap;

mod log;
mod text_trans;
mod tiny_map;
mod escape_str;
