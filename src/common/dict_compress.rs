use ::std::borrow::Cow;
use ::std::cell::LazyCell;
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::iter::Cloned;
use ::std::iter::FlatMap;
use ::std::process::Output;
use ::std::slice::Iter;
use ::std::sync::LazyLock;

use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

use crate::common::{INDX, TextTransformation};
use crate::common::dict::{DerivationInfo, DICT, DictEntry, iter_snippets};
use crate::common::dict_derive::DerivationInfo;
use crate::common::text_trans::DictStr;
use crate::common::trie::Trie;
use crate::tilde_log;

thread_local! {
    static DICT_META: LazyCell<DictMeta> = LazyCell::new(DictMeta::new);
}

#[derive(Debug)]
struct DictMeta {
    dict: &'static [DictEntry],
    trie: Trie,
    entry_info: HashMap<DictStr, DerivationInfo>,
    //TODO @mark: fewer allocations?
}

impl DictMeta {
    fn new() -> Self {
        tilde_log!("initializing DictMeta (large) for string compression");
        let mut trie = Trie::new();
        for snip in iter_snippets(&DICT) {
            trie.push(snip)
        }
        let mut entry_info = HashMap::new();
        for (index, entry) in DICT.iter().enumerate() {
            let DictEntry::Snippet { snip, .. } = entry else { continue };
            entry_info.insert(
                DictStr::try_from(&**snip).expect("dict entry too long for array string"),
                DerivationInfo {
                    derived_text: DictStr::try_from(&**snip).expect("derivation too long for array string"),
                    original_index: index,
                    transformation: TextTransformation::new_noop(),
                    cost: 1, //TODO @mark:
                });
        }
        DictMeta {
            dict: &DICT,
            trie,
            entry_info,
        }
    }
}

pub fn compress_with_dict(text: &str) -> Vec<INDX> {
    let mut rem = text;
    let mut numbers = vec![];
    let mut prefix = String::new();
    let mut buffer = String::new();
    while !rem.is_empty() {
        DICT_META.with(|meta| {
            meta.trie.longest_prefix_with(rem, &mut prefix, &mut buffer);
            rem = &rem[prefix.len()..];
            if prefix.is_empty() {
                //TODO @mark: return Err instead of panic?
                panic!("cannot encode string because dictionary does not contain '{}'", rem.chars().next().unwrap())
            }
            let nrs = meta.entry_info.get(&DictStr::try_from(prefix.as_str()).expect("prefix too long for array string"))
                //TODO @mark: use str instead of DictStr above? ^
                .unwrap_or_else(|| panic!("prefix not in dictionary: '{prefix}'"))
                .original_index
                .try_into().expect("index does not fit in type");
            numbers.push(nrs)
        })
    }
    numbers
}

#[cfg(test)]
mod compress_decode {
    use crate::common::dict::TEST_POEM;
    use crate::common::dict_lookup::lookup_alloc;

    use super::*;

    #[test]
    fn decode_random_nrs() {
        let mut nrs = (0 .. 1000).collect::<Vec<_>>();
        let text = lookup_alloc(&nrs);
        let compress = compress_with_dict(&text);
        assert!(compress.len() < nrs.len())
    }

    #[test]
    fn compress_poem() {
        let nrs = compress_with_dict(TEST_POEM);
        assert!(nrs.len() < 1);
        let text = lookup_alloc(&nrs);
        assert_eq!(text, TEST_POEM);
    }
}

#[cfg(test)]
mod compression {
    use super::*;

    #[test]
    fn simple_text_compression() {
        let nrs = compress_with_dict("hello world, this is a test");
        assert_eq!(nrs.len(), 16);
    }

    //TODO @mark: test more, e.g. symbols, caps
}
