use ::std::fmt;

use crate::{Value, Values};
use crate::exec::{dispatch_op, new_small_stack};
use crate::exec::Stack;
use crate::op::Op;

/// Which type of capture
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptureType {
    Unary(Op),
    BinaryFreeDeep(Op, Value),
    BinaryFreeTop(Op, Value),
    TernaryFreeDeep(Op, Value, Value),
    TernaryFreeMiddle(Op, Value, Value),
    TernaryFreeTop(Op, Value, Value),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Func {
    items: Vec<CaptureType>,
}

impl Func {
    pub fn new() -> Self {
        Func { items: Vec::with_capacity(4) }
    }

    pub fn run_on_single(&self, initial_stack_value: Value) -> Values {
        let mut stack = new_small_stack();
        stack.push(initial_stack_value);
        dispatch_op();
        todo!()
    }

    pub fn with_unary(mut self, op: Op) -> Self {
        self.items.push(CaptureType::Unary(op));
        self
    }

    pub fn with_bin_deep(mut self, op: Op, top: Value) -> Self {
        self.items.push(CaptureType::BinaryFreeDeep(op, top));
        self
    }

    pub fn with_bin_top(mut self, op: Op, deep: Value) -> Self {
        self.items.push(CaptureType::BinaryFreeTop(op, deep));
        self
    }

    pub fn with_tern_deep(mut self, op: Op, middle: Value, top: Value) -> Self {
        self.items.push(CaptureType::TernaryFreeDeep(op, middle, top));
        self
    }

    pub fn with_tern_middle(mut self, op: Op, deep: Value, top: Value) -> Self {
        self.items.push(CaptureType::TernaryFreeMiddle(op, deep, top));
        self
    }

    pub fn with_tern_top(mut self, op: Op, deep: Value, middle: Value) -> Self {
        self.items.push(CaptureType::TernaryFreeTop(op, deep, middle));
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
                CaptureType::Unary(op) => write!(f, "_ {op:?}")?,
                CaptureType::BinaryFreeDeep(op, top) => write!(f, "(_ {top:?} {op:?})")?,
                CaptureType::BinaryFreeTop(op, deep) => write!(f, "({deep:?} _ {op:?})")?,
                CaptureType::TernaryFreeDeep(op, middle, top) => write!(f, "(_ {middle:?} {top:?} {op:?})")?,
                CaptureType::TernaryFreeMiddle(op, deep, top) => write!(f, "({deep:?} _ {top:?} {op:?})")?,
                CaptureType::TernaryFreeTop(op, deep, middle) => write!(f, "({deep:?} {middle:?} _ {op:?})")?,
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}