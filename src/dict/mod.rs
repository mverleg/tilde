pub use self::compress::compress_with_dict;
pub use self::dict_str::CowDictStr;
pub use self::dict_str::DictStr;
pub use self::dict_str::DictStrContent;
pub use self::dict_str::LONGEST_DICT_ENTRY_BYTES;
pub use self::entry::DICT;
pub use self::entry::DictEntry;
pub use self::entry::INDX;
pub use self::lookup::lookup_buffer;

mod entry;
mod lookup;
mod compress;
mod derive;
mod dict_str;
mod trie_data;
mod prefix_data;
mod trie_original;
