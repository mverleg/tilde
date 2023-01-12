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

use crate::common::dict::iter_snippets;
use crate::common::dict::DictEntry;
use crate::common::dict::DICT;
use crate::common::dict_derive::with_derived_dict_entries;
use crate::common::dict_derive::DerivationInfo;
use crate::common::text_trans::DictStr;
use crate::common::trie_data::Trie;
use crate::common::TextTransformation;
use crate::common::INDX;
use crate::tilde_log;

thread_local! {
    static DICT_META: LazyCell<DictMeta> = LazyCell::new(DictMeta::new);
}

type ExtIndx = u32;

#[derive(Debug)]
struct DictMeta {
    base_dict: &'static [DictEntry],
    extended_dict: Vec<DerivationInfo>,
    trie: Trie<ExtIndx>,
    //TODO @mark: fewer allocations?
}

impl DictMeta {
    fn new() -> Self {
        tilde_log!("initializing DictMeta (large) for string compression");
        let extended_dict = with_derived_dict_entries(&DICT);
        let mut trie = Trie::new();
        for (index, snip) in extended_dict.iter().enumerate() {
            trie.push(snip.derived_text.as_ref(), index.try_into().expect("extended dict too large to find index"))
        }
        DictMeta {
            base_dict: &DICT,
            extended_dict,
            trie,
        }
    }
}

pub fn compress_with_dict(text: &str) -> Vec<INDX> {
    let mut rem = text;
    let mut numbers = Vec::new();
    let mut buffer = Vec::new();
    DICT_META.with(|meta| {
        while !rem.is_empty() {
            meta.trie.all_prefixes_cloned_of(rem, &mut buffer);
            //eprintln!("> {} | for: {}", buffer.iter().map(|c| format!("{:?}", meta.extended_dict[*c as usize])).collect::<Vec<_>>().join(" / "), rem);  //TODO @mark: TEMPORARY! REMOVE THIS!
            let deriv_index = *buffer.last()
                .unwrap_or_else(|| panic!("did not find snippet for {}", rem.chars().next().unwrap()));
            let deriv = &meta.extended_dict[deriv_index as usize];
            numbers.push(deriv.original_index.try_into().expect("could not convert usize into index"));
            numbers.extend(deriv.transformation.operation_indices());
            //let DictEntry::Snippet { snip: prefix, capitalize_next } = meta.base_dict[prefix_index as usize] else { todo!() };
            //eprintln!("prefix: {prefix}");  //TODO @mark: TEMPORARY! REMOVE THIS!
            rem = &rem[deriv.derived_text.as_ref().len()..];
            // if prefix.is_empty() {
            //     //TODO @mark: return Err instead of panic?
            //     panic!("cannot encode string because dictionary does not contain '{}'", rem.chars().next().unwrap())
            // }
            // let nrs = meta.entry_info.get(&DictStr::try_from(prefix.as_str()).expect("prefix too long for array string"))
            // TODO @mark: use str instead of DictStr above? ^
            // .unwrap_or_else(|| panic!("prefix not in dictionary: '{prefix}'"))
            // .original_index
            // .try_into().expect("index does not fit in type");
            //numbers.push(nrs)
        }
    });
    numbers
}

#[cfg(test)]
mod compress_decode {
    use super::*;
    use crate::common::dict::TEST_POEM;
    use crate::common::dict_lookup::lookup_alloc;

    #[test]
    fn decode_random_nrs() {
        let mut nrs = (0..1000).collect::<Vec<_>>();
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
