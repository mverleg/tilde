use crate::Value;
use crate::Values;

pub fn new_small_stack() -> Values {
    Values::new()
}

pub fn new_large_stack() -> Vec<Value> {
    Vec::new()
}

pub trait Stack {

    fn push(&mut self, value: Value);

    fn pop(&mut self) -> Option<Value>;
}

impl Stack for Vec<Value> {

    fn push(&mut self, value: Value) {
        self.push(value)
    }

    fn pop(&mut self) -> Option<Value> {
        self.pop()
    }
}

impl Stack for Values {

    fn push(&mut self, value: Value) {
        self.push(value)
    }

    fn pop(&mut self) -> Option<Value> {
        self.pop()
    }
}