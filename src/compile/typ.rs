#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Typ {
    Number,
    Text,
    List(Box<Typ>),
    Function,
}
