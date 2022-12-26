#![allow(warnings)]
#![allow(clippy::all)]

//TODO @mark: add a pop-first operation
//TODO @mark: maybe a reverse operation?

use crate::common::TextTransformation;

pub type INDX = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictEntry {
    Snippet { snip: &'static str, capitalize_next: bool, },
    Backspace,
    CapitalizeFirst,
    CapitalizeAll,
}

#[derive(Debug)]
pub struct DerivationInfo {
    pub derived_text: &'static str,
    pub original_index: usize,
    pub transformation: TextTransformation,
    pub cost: usize,
}

#[inline]
const fn s(snip: &'static str) -> DictEntry {
    DictEntry::Snippet{ snip, capitalize_next: false, }
}
//noinspection RsFunctionNaming
#[inline]
const fn S(snip: &'static str) -> DictEntry {
    DictEntry::Snippet{ snip, capitalize_next: true, }
}

include!(concat!(env!("OUT_DIR"), "/dict_init.rs"));

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

    use super::*;

    #[test]
    fn first_is_whitespace() {
        assert_eq!(DICT.get(1).copied(), Some(s(" ")), "first entry should be space (maybe stripped by editor?)");
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
