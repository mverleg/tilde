pub use self::base64::b64_decode;
pub use self::base64::b64_encode;
pub use self::escape_str::escape_for_string;
pub use self::escape_str::is_safe_for_string;
pub use self::log::log;
pub use self::text_trans::OpIndices;
pub use self::text_trans::SnipOrChar;
pub use self::text_trans::TextTransformation;
pub use self::text_trans::UNICODE_MAGIC_INDX;
pub use self::tiny_map::TinyMap;

mod log;
mod text_trans;
mod tiny_map;
mod escape_str;
mod base64;

