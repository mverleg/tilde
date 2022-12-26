use ::std::borrow::Cow;
use ::std::collections::HashMap;
use ::std::iter::Cloned;
use ::std::iter::FlatMap;
use ::std::slice::Iter;
use ::std::sync::{Arc, Mutex, RwLock};
use ::std::sync::LazyLock;

use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

use crate::common::dict::{DICT, DictEntry, INDX, SnipCombi};
use crate::common::trie::Trie;

//TODO @mark: change to smaller index type

pub fn decode_with_dict(nrs: &[INDX]) -> String {
    let mut buffer = String::new();
    for nr in nrs {
        let snippet = DICT.snippet_index.get(*nr as usize)
            .unwrap_or_else(|| panic!("dictionary has no item nr {nr}"));
        match snippet {
            DictEntry::Snippet(text) => {
                buffer.push_str(*text);
            },
            DictEntry::Backspace => { buffer.pop(); },
            DictEntry::CapitalizeFirst => todo!(),  //TODO @mark:
            DictEntry::CapitalizeAll => todo!(),  //TODO @mark:
        }
    }
    buffer
}

pub fn compress_with_dict(text: &str) -> Vec<INDX> {
    let mut rem = text;
    let mut numbers = vec![];
    let mut prefix = String::new();
    let mut buffer = String::new();
    while !rem.is_empty() {
        DICT.ext_prefix_tree.longest_prefix_with(rem, &mut prefix, &mut buffer);
        if prefix.is_empty() {
            //TODO @mark: return Err instead of panic?
            panic!("cannot encode string because dictionary does not contain '{}'", rem.chars().next().unwrap())
        }
        rem = &rem[prefix.len()..];
        let nrs = DICT.ext_snippet_positions.get(prefix.as_str())
            .unwrap_or_else(|| panic!("prefix not in dictionary: '{prefix}'"))
            .into_iter().map(|nr| *nr).collect::<SnipCombi>();
        numbers.extend(nrs)
    }
    //numbers
    todo!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

#[cfg(test)]
mod compress_decode {
    use super::*;

    #[test]
    fn decode_random_nrs() {
        let mut nrs = (0 .. 1000).collect::<Vec<_>>();
        let text = decode_with_dict(&nrs);
        let compress = compress_with_dict(&text);
        assert!(compress.len() < nrs.len())
    }

    #[test]
    fn compress_poem() {
        let nrs = compress_with_dict(TEST_POEM);
        assert!(nrs.len() < 1);
        let text = decode_with_dict(&nrs);
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
