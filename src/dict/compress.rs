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
use ::tinyvec::ArrayVec;

use crate::common::UNICODE_MAGIC_INDX;
use crate::dict::Cost;
use crate::dict::derive::DerivationInfo;
use crate::dict::derive::with_derived_dict_entries;
use crate::dict::DICT;
use crate::dict::DictEntry;
use crate::dict::DictIx;
use crate::dict::LONGEST_DICT_ENTRY_BYTES;
use crate::dict::lookup::lookup_alloc;
use crate::dict::lookup_buffer;
use crate::dict::MAX_TEXT_TRANSFORMS;
use crate::dict::prefix_data::PrefixMap;
//use crate::tilde_gen_md_docs;
//TODO @mark: ^ fix and enable `gen`
use crate::tilde_log;

thread_local! {
    static DICT_META: LazyCell<DictMeta> = LazyCell::new(DictMeta::new);
}

type CostSum = u32;
type ExtIx = u32;
type ExtEntryIxs = ArrayVec<[DictIx; MAX_TEXT_TRANSFORMS + 1]>;

#[derive(Debug)]
struct DictMeta {
    base_dict: &'static [DictEntry],
    extended_dict: Vec<DerivationInfo>,
    prefix_map: PrefixMap<ExtIx>,
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
    cost_from: CostSum,
    compressed_nr: ExtEntryIxs,
    snippet_len: u8,
    //TODO @mark: smaller size?
}

pub fn compress_with_dict(text: &str) -> Vec<DictIx> {
    if text.is_empty() {
        return Vec::new();
    }
    let mut reverse_chars = text.chars().collect::<Vec<char>>();
    reverse_chars.reverse();
    let mut minimums = initialize_minimums(text);
    let mut snippet_options_buffer = Vec::new();
    let mut len_from_here = text.len();
    tilde_log!("starting compression for '{}' (length {})", text.lines().next().unwrap(), text.len());
    DICT_META.with(|meta| {
        for letter in reverse_chars {
            // Find the cheapest from here until end
            len_from_here -= letter.len_utf8();
            meta.prefix_map.all_prefixes_cloned_of(&text[len_from_here..], &mut snippet_options_buffer);
            if snippet_options_buffer.is_empty() {
                //TODO @mark: ^
                let best_result = create_utf_lookup(letter, &minimums[len_from_here..]);
                tilde_log!("compressed char {} from slice '{}' using utf8 lookup (cost {}) because no dict entry matches",
                    letter, &text[len_from_here..], best_result.cost_from);
                minimums[len_from_here] = best_result;
            } else {
                tilde_log!("compressing slice '{}' using {} dict entries that share a prefix, e.g.: {}", &text[len_from_here..], snippet_options_buffer.len(),
                    snippet_options_buffer.iter().take(4).map(|ix| format!("{}({})", meta.extended_dict[*ix as usize].derived_text.as_ref(), meta.extended_dict[*ix as usize].cost)).collect::<Vec<String>>().join(", "));
                let best_result = select_best_match(&snippet_options_buffer, &minimums[len_from_here..], &meta.extended_dict);
                minimums[len_from_here] = best_result;
            }
        }
        debug_assert!(len_from_here == 0);
    });
    collect_cheapest_result(text, &minimums)
}

fn initialize_minimums(text: &str) -> Vec<BestSoFar> {
    // minimums are indexed by byte position instead of character position, leaving some gaps which is fine
    let worst = BestSoFar { cost_from: CostSum::MAX, compressed_nr: ExtEntryIxs::new(), snippet_len: 1 };
    vec![worst; text.len()]
}

fn create_utf_lookup(letter: char, minimums_from: &[BestSoFar]) -> BestSoFar {
    let mut ops = ExtEntryIxs::new();
    ops.push((letter as u32).try_into().expect("unicode lookup value too large for index data type"));
    ops.push(UNICODE_MAGIC_INDX);
    let snippet_len = letter.len_utf8();
    let continuation_cost = minimums_from.get(snippet_len)
        .map(|next| next.cost_from)
        .unwrap_or(0);
    BestSoFar {
        cost_from: continuation_cost + 2,  //TODO @mark: not +2 but real cost
        compressed_nr: ops,
        snippet_len: snippet_len as u8
    }
}

fn select_best_match(options: &[ExtIx], minimums_from: &[BestSoFar], extended_dict: &[DerivationInfo]) -> BestSoFar {
    let mut minimum = minimums_from[0];
    for option in options {
        let mut ops = ExtEntryIxs::new();
        let derivation_info = &extended_dict[*option as usize];
        ops.push(derivation_info.original_index.try_into().expect("could not convert to index from usize"));
        ops.extend(derivation_info.transformation.operation_indices());
        let snippet_len = derivation_info.derived_text.as_ref().len();
        //TODO @mverleg: this could also lookup the string, if it makes it faster to initialize the meta dict
        debug_assert!(snippet_len >= 1, "no snippet for ops: {ops}");
        let continuation_cost = minimums_from.get(snippet_len)
            .map(|next| next.cost_from)
            .unwrap_or(0);
        let cost_from = continuation_cost + derivation_info.cost as CostSum;
        if cost_from >= minimum.cost_from {
            continue
        }
        minimum = BestSoFar {
            cost_from,
            compressed_nr: ops,
            snippet_len: snippet_len as u8
        };
    }
    tilde_log!(" selected '{}' (#ops {}, #char {}, cost {}-{})", lookup_alloc(&minimum.compressed_nr), minimum.compressed_nr.len(),
        minimum.snippet_len, minimum.cost_from, minimums_from.get(minimum.snippet_len as usize).map(|bsf| bsf.cost_from).unwrap_or(0));
    minimum
}

fn collect_cheapest_result(text: &str, minimums: &[BestSoFar]) -> Vec<DictIx> {
    let mut i = 0;
    let mut numbers = Vec::new();
    while i < text.len() {
        debug_assert!(minimums[i].cost_from < CostSum::MAX, "index {i} or later one not updated");
        numbers.extend(&minimums[i].compressed_nr);
        i += minimums[i].snippet_len as usize;
    }
    numbers
}

#[cfg(test)]
mod compress_decode {
    use crate::compile::{Closer, encode_uint_vec};
    use crate::dict::entries::TEST_POEM;
    use crate::dict::lookup::lookup_alloc;

    use super::*;

    #[test]
    fn decode_random_nrs() {
        let n = 1000;
        let mut nrs = (500..(500 + n)).intersperse(0).collect::<Vec<_>>();
        let orig_enc = encode_uint_vec(&nrs, Closer::Text);
        let text = lookup_alloc(&nrs);
        let compress_ops = &compress_with_dict(&text);
        let compress = encode_uint_vec(compress_ops, Closer::Text);
        assert_eq!(lookup_alloc(&compress_ops), text);
        assert!(compress.len() < orig_enc.len(), "compression did not help: {} >= {}", compress.len(), nrs.len())
    }

    #[test]
    fn compress_special() {
        let sample = "hi there ♡©";
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
    use ::std::mem::size_of;
    use ::std::thread;

    use crate::run_tilde;

    use super::*;

    #[test]
    fn cost_size() {
        assert!(size_of::<Cost>() <= size_of::<CostSum>())
    }

    #[test]
    fn simple_text_compression() {
        let nrs = compress_with_dict("hello world, this is a test");
        assert_eq!(nrs.len(), 13);
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
