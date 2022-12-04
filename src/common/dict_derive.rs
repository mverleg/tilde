pub const MAX_BACKSPACE: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct DictDerivation {
    text: String,
    base_snippet: String,
    capitalize_self: CapitalizeKind,
    capitalize_next: bool,
    backspace_count: u8,
}

pub fn derivations(base_text: String) -> Vec<DictDerivation> {
    //TODO @mark: base_Text borrow? can DictDerivation still be without lifetime?
    let mut deriv = vec![];
    for cap in [CapitalizeKind::None, CapitalizeKind::First, CapitalizeKind::All] {
        let cap_text = cap.apply(&base_text);
        for bs in 0 ..= MAX_BACKSPACE {
            let mut bs_test = cap_text.clone();
            for _ in 0 .. bs {
                if let None = bs_test.pop() {
                    break
                }
            }
            deriv.push(DictDerivation {
                text: bs_test,
                base_snippet: base_text.clone(),
                capitalize_self: cap,
                capitalize_next: false,
                backspace_count: bs,
            });
        }
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
