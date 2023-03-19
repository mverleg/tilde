use ::std::fmt;

use crate::op::Op;
use crate::Value;

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
    //TODO @mark: tinyvec?
}

impl Func {
    pub fn new() -> Self {
        Func { items: Vec::with_capacity(4) }
    }

    pub fn with_unary(mut self, op: Op) -> Self {
        self.push(CaptureType::Unary(op));
        self
    }
    pub fn with_bin_left(mut self, op: Op, right: Value) -> Self {
        self.push(CaptureType::BinaryFreeDeep(op, right));
        self
    }
    pub fn with_bin_right(mut self, op: Op, left: Value) -> Self {
        self.push(CaptureType::BinaryFreeTop(op, left));
        self
    }
    pub fn with_tern_left(mut self, op: Op, middle: Value, right: Value) -> Self {
        self.push(CaptureType::TernaryFreeDeep(op, middle, right));
        self
    }
    pub fn with_tern_middle(mut self, op: Op, left: Value, right: Value) -> Self {
        self.push(CaptureType::TernaryFreeMiddle(op, left, right));
        self
    }
    pub fn with_tern_right(mut self, op: Op, left: Value, middle: Value) -> Self {
        self.push(CaptureType::TernaryFreeTop(op, left, middle));
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
                // CaptureType::Operation(op) => write!(f, "{op:?}")?,
                // CaptureType::Capture(val) => write!(f, "{val:?}")?,
                _ => todo!(),
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}