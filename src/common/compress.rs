use ::std::borrow::Cow;
use ::std::cell::LazyCell;
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::iter::Cloned;
use ::std::iter::FlatMap;
use ::std::slice::Iter;
use ::std::sync::{Arc, Mutex, RwLock};
use ::std::sync::LazyLock;

use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

use crate::common::dict::{DICT, DictEntry, iter_snippets};
use crate::common::INDX;
use crate::common::trie::Trie;
use crate::tilde_log;

const DICT_META: LazyCell<DictMeta> = LazyCell::new(DictMeta::new);

#[derive(Debug)]
struct DictMeta {
    dict: &'static [DictEntry],
    trie: Trie,
}

impl DictMeta {
    fn new() -> Self {
        tilde_log!("initializing DictMeta (large) for string compression");
        let mut trie = Trie::new();
        for snip in iter_snippets(&DICT) {
            trie.push(snip)
        }
        DictMeta {
            dict: &DICT,
            trie,
        }
    }
}

pub fn compress_with_dict(text: &str) -> Vec<INDX> {
    let f = DICT_META.trie.lookup("hello");
    // let mut rem = text;
    // let mut numbers = vec![];
    // let mut prefix = String::new();
    // let mut buffer = String::new();
    // while !rem.is_empty() {
    //     DICT.ext_prefix_tree.longest_prefix_with(rem, &mut prefix, &mut buffer);
    //     if prefix.is_empty() {
    //         //TODO @mark: return Err instead of panic?
    //         panic!("cannot encode string because dictionary does not contain '{}'", rem.chars().next().unwrap())
    //     }
    //     rem = &rem[prefix.len()..];
    //     let nrs = DICT.ext_snippet_positions.get(prefix.as_str())
    //         .unwrap_or_else(|| panic!("prefix not in dictionary: '{prefix}'"))
    //         .into_iter().map(|nr| *nr).collect::<SnipCombi>();
    //     numbers.extend(nrs)
    // }
    //numbers
    todo!()  //TODO @mark: TEMPORARY! REMOVE THIS!
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
