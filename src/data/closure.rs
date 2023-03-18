use ::std::fmt;

use crate::op::Op;
use crate::Value;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FuncItem {
    Operation(Op),
    Capture(Value),
}

#[derive(Clone, PartialEq, Hash)]
pub struct Func {
    items: Vec<FuncItem>,
    //TODO @mark: tinyvec?
}

impl Func {
    pub fn new() -> Self {
        Func { items: Vec::with_capacity(4) }
    }

    pub fn with_op(mut self, op: Op) -> Self {
        self.items.push(FuncItem::Operation(op));
        self
    }

    pub fn with_val(mut self, val: Value) -> Self {
        self.items.push(FuncItem::Capture(val));
        self
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(function)")
    }
}

impl fmt::Debug for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(arg ")?;
        let mut is_first = true;
        for item in &self.items {
            if is_first {
                is_first = false
            } else {
                write!(f, " ")?;
            }
            match item {
                FuncItem::Operation(op) => write!(f, "{op:?}")?,
                FuncItem::Capture(val) => write!(f, "{val:?}")?,
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}