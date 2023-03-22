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

impl Fork for CaptureType {
    fn fork(&self) -> Self {
        match self {
            CaptureType::Unary(op) => CaptureType::Unary(op.clone()),
            CaptureType::BinaryFreeDeep(op, top) => CaptureType::BinaryFreeDeep(op.clone(), top.fork()),
            CaptureType::BinaryFreeTop(op, deep) => CaptureType::BinaryFreeTop(op.clone(), deep.fork()),
            CaptureType::TernaryFreeDeep(op, middle, top) => CaptureType::TernaryFreeDeep(op.clone(), middle.fork(), top.fork()),
            CaptureType::TernaryFreeMiddle(op, deep, top) => CaptureType::TernaryFreeMiddle(op.clone(), deep.fork(), top.fork()),
            CaptureType::TernaryFreeTop(op, deep, middle) => CaptureType::TernaryFreeTop(op.clone(), deep.fork(), middle.fork()),
        }
    }
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

    pub fn with_unary(self, op: Op) -> Self {
        let mut new = self.fork_content();
        //TODO @mark: prevent clone (and sibling methods)
        new.push(CaptureType::Unary(op));
        Func { items: Rc::new(new) }
    }

    pub fn with_bin_deep(self, op: Op, top: Value) -> Self {
        let mut new = self.fork_content();
        new.push(CaptureType::BinaryFreeDeep(op, top));
        Func { items: Rc::new(new) }
    }

    pub fn with_bin_top(self, op: Op, deep: Value) -> Self {
        let mut new = self.fork_content();
        new.push(CaptureType::BinaryFreeTop(op, deep));
        Func { items: Rc::new(new) }
    }

    pub fn with_tern_deep(self, op: Op, middle: Value, top: Value) -> Self {
        let mut new = self.fork_content();
        new.push(CaptureType::TernaryFreeDeep(op, middle, top));
        Func { items: Rc::new(new) }
    }

    pub fn with_tern_middle(self, op: Op, deep: Value, top: Value) -> Self {
        let mut new = self.fork_content();
        new.push(CaptureType::TernaryFreeMiddle(op, deep, top));
        Func { items: Rc::new(new) }
    }

    pub fn with_tern_top(self, op: Op, deep: Value, middle: Value) -> Self {
        let mut new = self.fork_content();
        new.push(CaptureType::TernaryFreeTop(op, deep, middle));
        Func { items: Rc::new(new) }
    }

    fn fork_content(&self) -> Vec<CaptureType> {
        self.items.iter().map(|item| item.fork()).collect()
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