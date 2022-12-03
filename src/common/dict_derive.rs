
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum CapitalizeKind {
    None,
    First,
    All,
}

#[derive(Debug, Clone)]
pub struct DictCombi {
    text: String,
    base_snippet: &'static str,
    capitalize_self: CapitalizeKind,
    capitalize_next: bool,
    backspaced: u8,
    cost: u8,
}
