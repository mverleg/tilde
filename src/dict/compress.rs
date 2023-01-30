use ::std::borrow::Cow;
use ::std::cell::LazyCell;
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::iter::Cloned;
use ::std::iter::FlatMap;
use ::std::process::Output;
use ::std::slice::Iter;
use ::std::sync::LazyLock;
use ::std::time::Instant;

use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

use crate::common::{OpIndices, UNICODE_MAGIC_INDX};
use crate::dict::{DICT, DictEntry, INDX, LONGEST_DICT_ENTRY_BYTES, lookup_buffer};
use crate::dict::derive::{DerivationInfo, with_derived_dict_entries};
use crate::dict::prefix_data::PrefixMap;
use crate::{tilde_gen_md_docs, tilde_log};

thread_local! {
    static DICT_META: LazyCell<DictMeta> = LazyCell::new(DictMeta::new);
}

type ExtIndx = u32;

#[derive(Debug)]
struct DictMeta {
    base_dict: &'static [DictEntry],
    extended_dict: Vec<DerivationInfo>,
    prefix_map: PrefixMap<ExtIndx>,
    //TODO @mark: fewer allocations?
}

impl DictMeta {
    //TODO @mverleg: if this is still too slow, it could probably happen at compile time (but it's probably fine if under 200ms)
    fn new() -> Self {
        tilde_log!("initializing DictMeta (large) for string compression");
        let start = Instant::now();
        let extended_dict = with_derived_dict_entries(&DICT);
        let mut prefix_map = PrefixMap::with_capacity(extended_dict.len());
        for (index, snip) in extended_dict.iter().enumerate() {
            prefix_map.push(
                snip.derived_text.to_owned(),
                //TODO @mverleg: get rid of clone? impossible without lifetimes perhaps, but duplicate data with extended_dict, so it is wasteful
                index.try_into().expect("extended dict too large to find index"))
        }
        let duration = start.elapsed();
        tilde_log!("DictMeta has {} entries based on {} base entries, init took {} ms`", extended_dict.len(), DICT.len(), duration.as_millis());
        DictMeta {
            base_dict: &DICT,
            extended_dict: extended_dict.into_iter().collect::<Vec<_>>(),
            prefix_map,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct BestSoFar {
    score_from: usize,
    compressed_nr: OpIndices,
    snippet_len: usize,
    //TODO @mark: smaller size?
}

pub fn compress_with_dict(text: &str) -> Vec<INDX> {
    let rev_chars = text.chars().rev().collect::<Vec<char>>();
    let mut transformed_snippet = String::with_capacity(LONGEST_DICT_ENTRY_BYTES);
    let mut char_buffer = Vec::with_capacity(LONGEST_DICT_ENTRY_BYTES);
    let mut minimums = vec![BestSoFar { score_from: usize::MAX, compressed_nr: OpIndices::new(), snippet_len: 1, }; text.len()];
    let mut buffer = Vec::new();
    let mut tail_len = 0;
    DICT_META.with(|meta| {
        for (i, letter) in rev_chars.into_iter().enumerate() {
            // Find the cheapest from here until end
            tail_len += letter.len_utf8();
            meta.prefix_map.all_prefixes_cloned_of(&text[tail_len..], &mut buffer);
            if true || buffer.is_empty() {
                //TODO @mark: ^
                // Did not find a single entry that matches, in this case we fall back to unicode lookup.
                let mut ops = OpIndices::new();
                ops.push((letter  as u32).try_into().expect("unicode lookup value too large for index data type"));
                ops.push(UNICODE_MAGIC_INDX);
                let score = 1;  //TODO @mark:
                transformed_snippet.clear();
                lookup_buffer(&ops, &mut transformed_snippet, &mut char_buffer);
                let snippet_len = transformed_snippet.len();
                //TODO @mark: can this just be 1? ^
                minimums[i] = BestSoFar { score_from: score, compressed_nr: ops, snippet_len };
                tilde_log!("compress index {}-{i} using unicode {letter} (only one option)", text.len())
            } else {
                todo!()
            }


        }
        // while !rem.is_empty() {
        //     meta.prefix_map.all_prefixes_cloned_of(rem, &mut buffer);
        //     if ! buffer.is_empty() {
        //         while let Some(deriv_index) = buffer.last() {
        //             // Found entry in the derived dictionary, use the base snippet and any transformations
        //             let deriv = &meta.extended_dict[*deriv_index as usize];
        //             //eprintln!("for rem len = {} at '{}' found {} matches {}", rem.len(), rem.chars().next().unwrap(), //TODO @mark: TEMPORARY! REMOVE THIS!
        //             //          buffer.len(), buffer.iter().map(|nr| format!("{nr}='{}'", &meta.extended_dict[*nr as usize].derived_text.as_ref())).collect::<Vec<_>>().join(", "));  //TODO @mark: TEMPORARY! REMOVE THIS!
        //             numbers.push(deriv.original_index.try_into().expect("could not convert usize into index"));
        //             numbers.extend(deriv.transformation.operation_indices());
        //             rem = &rem[deriv.derived_text.as_ref().len()..];
        //         }
        //     } else {
        //         // Did not find a single entry that matches, the only choice is to use unicode lookup.
        //         // (Unicode lookup is only generated if normal dict fails, but that is fine because dict is almost always cheaper.)
        //         let letter = rem.chars().next().expect("first char must exist here");
        //         numbers.push((letter  as u32).try_into().expect("unicode lookup value too large for index data type"));
        //         numbers.push(UNICODE_MAGIC_INDX);
        //         rem = &rem[letter.len_utf8()..];
        //     }
        // }
    });
    dbg!(&minimums);
    let mut i = 0;
    let mut numbers = Vec::new();
    while i < text.len() {
        numbers.extend(&minimums[i].compressed_nr);
        i += minimums[i].snippet_len;
    }
    numbers
}

#[cfg(test)]
mod compress_decode {
    use crate::dict::entries::TEST_POEM;
    use crate::dict::lookup::lookup_alloc;

    use super::*;

    #[test]
    fn decode_random_nrs() {
        let n = 1000;
        let mut nrs = (500..(500 + n)).intersperse(0).collect::<Vec<_>>();
        let text = lookup_alloc(&nrs);
        let compress = compress_with_dict(&text);
        eprintln!("{}", nrs.iter().map(|nr| nr.to_string()).collect::<Vec<_>>().join(" "));  //TODO @mark: TEMPORARY! REMOVE THIS!
        eprintln!("{}", compress.iter().map(|nr| nr.to_string()).collect::<Vec<_>>().join(" "));  //TODO @mark: TEMPORARY! REMOVE THIS!
        assert!(compress.len() < nrs.len())
    }

    #[test]
    fn compress_special() {
        let sample = "hi ©©";
        let nrs = compress_with_dict(sample);
        assert!(nrs.len() > 1);
        let text = lookup_alloc(&nrs);
        assert_eq!(text, sample);
    }

    #[test]
    fn compress_poem() {
        let nrs = compress_with_dict(TEST_POEM);
        assert!(nrs.len() > 1);
        let text = lookup_alloc(&nrs);
        assert_eq!(text, TEST_POEM);
    }
}

#[cfg(test)]
mod compression {
    use ::std::thread;

    use crate::run_tilde;

    use super::*;

    #[test]
    fn simple_text_compression() {
        let nrs = compress_with_dict("hello world, this is a test");
        assert_eq!(nrs.len(), 15);
    }

    #[test]
    #[ignore]  //TODO @mark: remove this test?
    fn bench() {
        compress_with_dict("hello world, this is a test");
        let n = 100;
        let start = Instant::now();
        for _ in 0..n {
            thread::spawn(|| compress_with_dict("hello world, this is a test")).join().unwrap();
        }
        let duration = start.elapsed();
        println!("{} iterations in {} ms so {} ms/iter", n, duration.as_millis(), duration.as_millis() / n)
    }

    //TODO @mark: test more, e.g. symbols, caps
}
