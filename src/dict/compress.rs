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
use crate::dict::derive::DerivationInfo;
use crate::dict::derive::with_derived_dict_entries;
use crate::dict::DICT;
use crate::dict::DictEntry;
use crate::dict::INDX;
use crate::dict::LONGEST_DICT_ENTRY_BYTES;
use crate::dict::lookup_buffer;
use crate::dict::MAX_TEXT_TRANSFORMS;
use crate::dict::prefix_data::PrefixMap;
use crate::tilde_gen_md_docs;
use crate::tilde_log;

thread_local! {
    static DICT_META: LazyCell<DictMeta> = LazyCell::new(DictMeta::new);
}

pub type COST = u16;
type EXTINDX = u32;
type ExtEntryIndices = ArrayVec<[INDX; MAX_TEXT_TRANSFORMS + 1]>;

#[derive(Debug)]
struct DictMeta {
    base_dict: &'static [DictEntry],
    extended_dict: Vec<DerivationInfo>,
    prefix_map: PrefixMap<EXTINDX>,
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
    cost_from: COST,
    compressed_nr: ExtEntryIndices,
    snippet_len: u8,
    //TODO @mark: smaller size?
}

pub fn compress_with_dict(text: &str) -> Vec<INDX> {
    let rev_chars = text.chars().rev().collect::<Vec<char>>();
    let mut transformed_snippet = String::with_capacity(LONGEST_DICT_ENTRY_BYTES);
    let mut minimums = vec![BestSoFar { cost_from: COST::MAX, compressed_nr: ExtEntryIndices::new(), snippet_len: 1, }; text.len()];
    // only character boundaries in `minimums` will be used, that waste is acceptable
    let mut snippet_options_buffer = Vec::new();
    let mut tail_len = 0;
    tilde_log!("starting compression for {} (length {})", text, text.len());
    DICT_META.with(|meta| {
        for letter in rev_chars.into_iter() {
            // Find the cheapest from here until end
            meta.prefix_map.all_prefixes_cloned_of(&text[tail_len..], &mut snippet_options_buffer);
            if snippet_options_buffer.is_empty() {
                // Did not find a single entry that matches, in this case we fall back to unicode lookup.
                let mut ops = ExtEntryIndices::new();
                ops.push((letter  as u32).try_into().expect("unicode lookup value too large for index data type"));
                ops.push(UNICODE_MAGIC_INDX);
                let snippet_len = 1;
                let continuation_cost = if tail_len > snippet_len {
                    minimums[tail_len - snippet_len].cost_from
                } else {
                    0
                };
                let cost_from = continuation_cost + 2;  //TODO @mark: not +2 but real cost
                minimums[tail_len] = BestSoFar { cost_from, compressed_nr: ops, snippet_len: snippet_len as u8 };
                tilde_log!("compress index {} using unicode '{letter}' (only one option)", text.len() - tail_len)
            } else {
                for snip_op in &snippet_options_buffer {
                    let mut ops = ExtEntryIndices::new();
                    let derivation_info = &meta.extended_dict[*snip_op as usize];
                    ops.push(derivation_info.original_index.try_into().expect("could not convert to index from usize"));
                    ops.extend(derivation_info.transformation.operation_indices());
                    transformed_snippet.clear();
                    //lookup_buffer(&snip_op, &mut transformed_snippet, &mut char_buffer);
                    let snippet_len = derivation_info.derived_text.as_ref().len();
                    //TODO @mverleg: this could also lookup the string, if it makes it faster to initialize the meta dict
                    debug_assert!(snippet_len >= 1, "no snippet for ops: {ops}");
                    let continuation_cost = if tail_len > snippet_len {
                        debug_assert!(minimums[tail_len - snippet_len].cost_from < COST::MAX,
                            "previous entry ({}) not initialized", tail_len - snippet_len);
                        minimums[tail_len - snippet_len].cost_from
                    } else {
                        0
                    };
                    let cost_from = continuation_cost + derivation_info.cost;
                    if cost_from < minimums[tail_len].cost_from {
                        tilde_log!("compress index {}, found a cheaper option #{snip_op}='{}' (out of {}): {} < {}", text.len() - tail_len,
                            derivation_info.derived_text.as_ref(), snippet_options_buffer.len(), cost_from, minimums[tail_len].cost_from);
                        minimums[tail_len] = BestSoFar { cost_from, compressed_nr: ops, snippet_len: snippet_len as u8 };
                    } else {
                        tilde_log!("compress index {}, discarded more expensive option #{snip_op}='{}' (out of {}): {} >= {}", text.len() - tail_len,
                            derivation_info.derived_text.as_ref(), snippet_options_buffer.len(), cost_from, minimums[tail_len].cost_from);
                    }
                }
            }
            tail_len += minimums[tail_len].snippet_len as usize;
        }
    });
    for (q, min) in minimums.iter().enumerate() {  //TODO @mverleg: TEMPORARY! REMOVE THIS!
        println!("{q}\t{}\t{}\t{}", min.cost_from, min.compressed_nr, min.snippet_len)  //TODO @mverleg: TEMPORARY! REMOVE THIS!
    }
    let mut i = 0;
    let mut numbers = Vec::new();
    while i < text.len() {
        debug_assert!(minimums[i].cost_from > COST::MAX, "index {i} or later one not updated");
        numbers.extend(&minimums[i].compressed_nr);
        i += minimums[i].snippet_len as usize;
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
