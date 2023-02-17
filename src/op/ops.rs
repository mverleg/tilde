use ::std::array::IntoIter;
use ::std::borrow::Cow;

use crate::common::escape_for_string;
use crate::compile::encode_str;
use crate::compile::Letter;
use crate::Nr;
use crate::TildeRes;

//TODO @mark: delete file?

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    // Value
    Text(String),
    Number(Nr),
    // TODO @mverleg: ^ change to something exact

    // Unary
    Neg,
    Abs,
    Incr,
    Decr,

    // Binary math
    Plus,
    Minus,
    Mul,
    Div,
    IntDiv,
    Mod,

    // Comparison (binary)
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,

    // Binary logic
    And,
    Or,
    Nand,
    Xor,
    Impl,
}

impl Op {
    pub fn text(txt: impl Into<String>) -> Self {
        Op::Text(txt.into())
    }

    pub fn number(nr: impl Into<Nr>) -> Self {
        Op::Number(nr.into())
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Text(_) => "text",
            Self::Number(_) => "nr",

            Self::Neg => "neg",
            Self::Abs => "abs",
            Self::Incr => "incr",
            Self::Decr => "decr",

            Self::Plus => "plus",
            Self::Minus => "minus",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::IntDiv => "int-div",
            Self::Mod => "modulo",

            Self::Eq => "eq",
            Self::Neq => "neq",
            Self::Gt => "gt",
            Self::Gte => "gte",
            Self::Lt => "lt",
            Self::Lte => "lte",

            Self::And => "and",
            Self::Or => "or",
            Self::Nand => "nand",
            Self::Xor => "xor",
            Self::Impl => "impl",
        }
    }

    pub fn iter() -> IntoIter<Op, 23> {
        use self::Op::*;
        [
            Text("".to_owned()),
            Number(Nr::zero()),
            Neg,
            Abs,
            Incr,
            Decr,
            Plus,
            Minus,
            Mul,
            Div,
            IntDiv,
            Mod,
            Eq,
            Neq,
            Gt,
            Gte,
            Lt,
            Lte,
            And,
            Or,
            Nand,
            Xor,
            Impl,
        ].into_iter()
    }
}

#[cfg(test)]
mod tests {
    use ::std::mem::variant_count;

    use super::*;

    #[test]
    fn iter_is_complete() {
        assert_eq!(Op::iter().count(), variant_count::<Op>());
    }

    #[test]
    fn ops_iterable() {
        let mut names = String::with_capacity(4096);
        for op in Op::iter() {
            names.push_str(op.name())
        }
        assert!(names.len() > 50);
    }
}
