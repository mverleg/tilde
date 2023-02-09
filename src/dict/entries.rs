#![allow(warnings)]
#![allow(clippy::all)]

//TODO @mark: add a pop-first operation
//TODO @mark: maybe a reverse operation?

use ::std::iter::FlatMap;
use ::std::option::IntoIter;
use ::std::slice::Iter;

use ::strum_macros::EnumIter;

use crate::common::TextTransformation;

pub type Cost = u8;
pub type DictIx = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum DictEntry {
    Snippet { snip: &'static str, capitalize_next: bool, cost: Cost },
    Backspace,
    BackspaceFront,
    CapitalizeFirst,
    CapitalizeAll,
    Reverse,
    UnicodeLookup,
}

#[inline]
const fn s(snip: &'static str, cost: Cost) -> DictEntry {
    DictEntry::Snippet { snip, capitalize_next: false, cost }
}

//noinspection RsFunctionNaming
#[inline]
const fn S(snip: &'static str, cost: Cost) -> DictEntry {
    DictEntry::Snippet { snip, capitalize_next: true, cost }
}

include!(concat!(env!("OUT_DIR"), "/dict_init.rs"));

pub fn iter_snippets(dict: &'static [DictEntry]) -> impl Iterator<Item=(usize, &'static str)> {
    dict.iter()
        .enumerate()
        .flat_map(|(index, entry)| match *entry {
            DictEntry::Snippet { snip, capitalize_next, cost } => Some(snip),
            DictEntry::Backspace => None,
            DictEntry::BackspaceFront => None,
            DictEntry::CapitalizeFirst => None,
            DictEntry::CapitalizeAll => None,
            DictEntry::Reverse => None,
            DictEntry::UnicodeLookup => None,
        }.map(|e| (index, e)).into_iter())
}

#[cfg(test)]
pub(crate) static TEST_POEM: &'static str = "
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
mod dict_properties {
    use ::std::collections::HashSet;

    use ::strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn first_is_whitespace() {
        assert_eq!(DICT.get(1).copied(), Some(s(" ", 1)), "first entry should be space (maybe stripped by editor?)");
    }

    #[test]
    fn trailing_whitespace() {
        let trailing_whitespace_count = iter_snippets(&DICT)
            .filter(|entry| entry.1.ends_with(" "))
            .count();
        assert!(trailing_whitespace_count > 10, "quite some entries should have trailing space (maybe stripped by editor?)");
    }

    #[test]
    fn no_duplicates() {
        let mut seen = HashSet::new();
        for entry in iter_snippets(&DICT) {
            assert!(seen.insert(entry), "duplicate: {entry:?}");
        }
    }

    #[test]
    fn no_leftover_specials() {
        for entry in iter_snippets(&DICT) {
            if entry.1.matches("$").count() >= 2 {
                panic!("unparsed magic value: {entry:?}")
            }
        }
    }

    #[test]
    fn all_specials_encountered() {
        let seen = DICT.iter()
            .filter(|entry| !matches!(entry, DictEntry::Snippet { .. }))
            .collect::<HashSet<_>>();
        for expect in DictEntry::iter() {
            if matches!(expect, DictEntry::Snippet { .. }) {
                continue;
            }
            assert!(seen.contains(&expect), "expected in dict: {expect:?}");
        }
    }
}

#[cfg(test)]
mod cost {
    use crate::compile::encode_snippet_len_estimate;

    use super::*;
    use super::DICT;

    #[test]
    fn cost_matches_dict_position() {
        for (pos, entry) in DICT.iter().enumerate() {
            assert_eq!(encode_snippet_len_estimate(pos.try_into().unwrap()), entry.cost().try_into().unwrap());
        }
    }
}
