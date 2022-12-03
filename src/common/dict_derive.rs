
pub const MAX_BACKSPACE: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapitalizeKind {
    None,
    First,
    All,
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
                base_snippet: todo!(), //base_text,
                capitalize_self: *cap,
                capitalize_next: false,
                backspace_count: bs,
            });
        }
    }
    deriv
}
