use ::std::borrow::Cow;

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
    base_snippet: &'static str,
    capitalize_self: CapitalizeKind,
    capitalize_next: bool,
    backspace_count: u8,
}

pub fn derivations(base_text: &str) -> Vec<DictDerivation> {
    let mut deriv = vec![];
    for cap in &[] {
        for bs in 0 ..= MAX_BACKSPACE {
            let text = base_text.to_owned();
            deriv.push(DictDerivation {
                text,
                base_snippet: todo!(),
                capitalize_self: *cap,
                capitalize_next: false,
                backspace_count: bs,
            });
        }
    }
    deriv
}
