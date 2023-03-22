use ::std::fmt;
use std::rc::Rc;
use crate::data::Fork;

use crate::exec::{dispatch_binary, Executor, new_small_stack};
use crate::exec::Stack;
use crate::op::Op;
use crate::Value;
use crate::Values;

/// Which type of capture
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CaptureType {
    Unary(Op),
    BinaryFreeDeep(Op, Value),
    BinaryFreeTop(Op, Value),
    TernaryFreeDeep(Op, Value, Value),
    TernaryFreeMiddle(Op, Value, Value),
    TernaryFreeTop(Op, Value, Value),
}

#[derive(PartialEq, Eq, Hash)]
pub struct Func {
    items: Rc<Vec<CaptureType>>,
}

impl Func {
    pub fn new() -> Self {
        Func { items: Rc::new(Vec::with_capacity(4)) }
    }

    pub fn run_on_single(&self, initial_stack_value: Value) -> Values {
        let mut stack = new_small_stack();
        stack.push(initial_stack_value);
        self.run_on_stack(&mut stack);
        stack
    }

    pub fn run_on_stack(&self, stack: &mut impl Stack) {
        for cap in &*self.items {
            let free_value = stack.pop();
            let res = match cap {
                CaptureType::Unary(op) => todo!(),
                CaptureType::BinaryFreeDeep(op, top) => {
                    let Executor::Binary(ex) = op.as_executor() else {
                        unreachable!();  //TODO @mark: really?
                    };
                    dispatch_binary(ex, Some(top.fork()), free_value)
                    //TODO @mark: get rid of top clone
                }
                CaptureType::BinaryFreeTop(op, deep) => {
                    let Executor::Binary(ex) = op.as_executor() else {
                        unreachable!();  //TODO @mark: really?
                    };
                    dispatch_binary(ex, free_value, Some(deep.fork()))
                    //TODO @mark: get rid of deep clone
                }
                CaptureType::TernaryFreeDeep(op, _, _) => todo!(),
                CaptureType::TernaryFreeMiddle(op, _, _) => todo!(),
                CaptureType::TernaryFreeTop(op, _, _) => todo!(),
            };
            stack.push_all(res)
        }
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

    pub fn fork(&self) -> Func {
        Func { items: self.items.clone() }
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
        for item in &*self.items {
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