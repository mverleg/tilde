pub const MAX_BACKSPACE: u8 = 3;

//TODO @mark: reverse

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapitalizeKind {
    None,
    First,
    All,
}

impl CapitalizeKind {
    pub fn apply(&self, input: &str) -> String {
        match self {
            CapitalizeKind::None => input.to_owned(),
            CapitalizeKind::First => {
                let mut iter = input.chars();
                let mut text = match iter.next() {
                    Some(c) => toggle_case(c),
                    None => return input.to_owned(),
                };
                iter.for_each(|c| text.push(c));
                text
            }
            CapitalizeKind::All => input.chars()
                .map(toggle_case)
                .collect(),
        }
    }
}

fn toggle_case(input: char) -> String {
    //TODO @mark: so many allocations... (because upper case may be several chars long)
    let up = input.to_uppercase().collect();
    if input.to_string() != up {
        return up;
    }
    input.to_lowercase().collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapDerivationSteps {
    capitalize_self: CapitalizeKind,
}

impl CapDerivationSteps {
    pub fn from_repr(repr: char) -> Self {
        match repr {
            'a' => CapDerivationSteps { capitalize_self : CapitalizeKind::None },
            'b' => CapDerivationSteps { capitalize_self : CapitalizeKind::First },
            'c' => CapDerivationSteps { capitalize_self : CapitalizeKind::All },
            _ => unimplemented!(),
        }
    }

    pub fn cost(&self) -> usize {
        1  //TODO @mark: TEMPORARY! REMOVE THIS!
    }

    pub fn to_repr(&self) -> char {
        match self.capitalize_self {
            CapitalizeKind::None => 'a',
            CapitalizeKind::First => 'b',
            CapitalizeKind::All => 'c',
        }
    }
}

pub fn cap_derivations(base_text: &str) -> Vec<DictDerivation> {
    //TODO @mark: base_Text borrow? can DictDerivation still be without lifetime?
    let mut deriv = vec![];
    eprintln!("todo remove CapitalizeKind::None here");
    for cap in [CapitalizeKind::None, CapitalizeKind::First, CapitalizeKind::All] {
        let cap_text = cap.apply(base_text).clone();
        deriv.push(DictDerivation {
            text: cap_text,
            steps: CapDerivationSteps {
                capitalize_self: cap,
            }
        });
    }
    deriv
}

#[cfg(test)]
mod capitalize {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(CapitalizeKind::None.apply(""), "");
        assert_eq!(CapitalizeKind::First.apply(""), "");
        assert_eq!(CapitalizeKind::All.apply(""), "");
    }

    #[test]
    fn none() {
        assert_eq!(CapitalizeKind::None.apply("a"), "a");
        assert_eq!(CapitalizeKind::None.apply("abc"), "abc");
        assert_eq!(CapitalizeKind::None.apply("ABC"), "ABC");
        assert_eq!(CapitalizeKind::None.apply("🦀"), "🦀");
    }

    #[test]
    fn first() {
        assert_eq!(CapitalizeKind::First.apply("a"), "A");
        assert_eq!(CapitalizeKind::First.apply("abc"), "Abc");
        assert_eq!(CapitalizeKind::First.apply("ABC"), "aBC");
        assert_eq!(CapitalizeKind::First.apply("🦀"), "🦀");
    }

    #[test]
    fn all() {
        assert_eq!(CapitalizeKind::All.apply("A"), "a");
        assert_eq!(CapitalizeKind::All.apply("abc"), "ABC");
        assert_eq!(CapitalizeKind::All.apply("ABC"), "abc");
        assert_eq!(CapitalizeKind::All.apply("🦀"), "🦀");
    }
}

#[cfg(test)]
mod encoding {
use ::strum::IntoEnumIterator;

    use crate::common::dict::DictEntry;

    use super::*;

    #[test]
    fn all_specials_encountered() {
        for expect in CapDerivationSteps::iter() {
            if matches!(expect, DictEntry::Snippet { .. }) {
                continue
            }
            assert!(seen.contains(&expect), "expected in dict: {expect:?}");
        }
    }
}
