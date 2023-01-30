use ::std::borrow::Cow;

use ::strum_macros::EnumIter;

use crate::common::escape_for_string;
use crate::op::typ::Typ;

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum Op {
    // Value
    Text(String),
    Number(f64),
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

    pub fn long_code(&self) -> Cow<str> {
        match self {
            Op::Text(text) => Cow::Owned(format!("\"{}\"", escape_for_string(text))),
            Op::Number(number) => Cow::Owned(format!("{}", number)),
            _ => todo!("impl long code"),
        }
    }

    pub fn gold_code(&self) -> Cow<str> {
        match self {
            Op::Text(text) => Cow::Owned(format!("todo-golf-text:{}", text)),
            Op::Number(number) => Cow::Owned(format!("todo-golf-number:{}", number)),
            _ => todo!("impl long code"),
        }
    }
}

#[cfg(test)]
mod tests {
    use ::strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn ops_iterable() {
        let mut names = String::with_capacity(4096);
        for op in Op::iter() {
            names.push_str(op.name())
        }
        assert!(names.len() > 50);
    }
}
