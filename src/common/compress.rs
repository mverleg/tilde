use ::std::borrow::Cow;
use ::std::collections::HashMap;
use ::std::iter::Cloned;
use ::std::iter::FlatMap;
use ::std::slice::Iter;
use ::std::sync::{Arc, Mutex, RwLock};
use ::std::sync::LazyLock;

use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

use crate::common::dict::{DICT, DictEntry};
use crate::common::trie::Trie;
use crate::UINT;

pub fn decode_with_dict(nrs: &[UINT]) -> String {
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

pub fn compress_with_dict(text: &str) -> Vec<UINT> {
    let mut rem = text;
    let mut numbers = vec![];
    let mut prefix = String::new();
    let mut buffer = String::new();
    while !rem.is_empty() {
        DICT.prefix_tree.longest_prefix_with(rem, &mut prefix, &mut buffer);
        if prefix.is_empty() {
            //TODO @mark: return Err instead of panic?
            panic!("cannot encode string because dictionary does not contain '{}'", rem.chars().next().unwrap())
        }
        rem = &rem[prefix.len()..];
        let nr = *DICT.snippet_positions.get(prefix.as_str())
            .unwrap_or_else(|| panic!("prefix not in dictionary: '{prefix}'")) as UINT;
        numbers.push(nr)
    }
    numbers
}

#[cfg(test)]
static TEST_POEM: &'static str = "
When you get what you want in your struggle for pelf,
And the world makes you King for a day,
Then go to the mirror and look at yourself,
And see what that guy has to say.

For it isn't your Father, or Mother, or Wife,
Who judgement upon you must pass.
The feller whose verdict counts most in your life
Is the guy staring back from the glass.

He's the feller to please, never mind all the rest,
For he's with you clear up to the end,
And you've passed your most dangerous, difficult test
If the guy in the glass is your friend.

You may be like Jack Horner and 'chisel' a plum,
And think you're a wonderful guy,
But the man in the glass says you're only a bum
If you can't look him straight in the eye.

You can fool the whole world down the pathway of years,
And get pats on the back as you pass,
But your final reward will be heartaches and tears
If you've cheated the guy in the glass.

© 1934 - Dale Wimbrow (1895-1954)";

#[cfg(test)]
mod lookup {
    use ::std::collections::HashSet;

    use crate::common::dict::{DICT, dict_iter};
    use crate::common::dict::dict_iter_snippets;

    use super::*;

    #[test]
    fn first_is_whitespace() {
        assert_eq!(DICT.snippet_index.get(1).copied(), Some(DictEntry::new_snippet(" ", false)), "first entry should be space (maybe stripped by editor?)");
    }

    #[test]
    fn trailing_whitespace() {
        let trailing_whitespace_count = dict_iter_snippets()
            .filter(|entry| entry.ends_with(" "))
            .count();
        assert!(trailing_whitespace_count > 10, "quite some entries should have trailing space (maybe stripped by editor?)");
    }

    #[test]
    fn no_duplicates() {
        let mut seen = HashSet::new();
        for entry in dict_iter() {
            assert!(seen.insert(entry), "duplicate: {entry:?}");
        }
    }

    #[test]
    fn no_leftover_specials() {
        for entry in dict_iter_snippets() {
            if entry.matches("$").count() >= 2 {
                panic!("unparsed magic value: {entry:?}")
            }
        }
    }

    #[test]
    fn all_specials_encountered() {
        let seen = dict_iter()
            .filter(|entry| !matches!(entry, DictEntry::Snippet { .. }))
            .collect::<HashSet<_>>();
        for expect in DictEntry::iter() {
            if matches!(expect, DictEntry::Snippet { .. }) {
                continue
            }
            assert!(seen.contains(&expect), "expected in dict: {expect:?}");
        }
    }
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
