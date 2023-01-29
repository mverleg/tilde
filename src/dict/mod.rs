pub use self::compress::compress_with_dict;
pub use self::dict_str::CowDictStr;
pub use self::dict_str::DictStr;
pub use self::dict_str::DictStrContent;
pub use self::dict_str::LONGEST_DICT_ENTRY_BYTES;
pub use self::entries::DICT;
pub use self::entries::DictEntry;
pub use self::entries::INDX;
pub use self::lookup::lookup_buffer;

mod entries;
mod lookup;
mod compress;
mod derive;
mod dict_str;
mod trie_data;
mod prefix_data;
mod trie_original;
