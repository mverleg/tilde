use ::std::fmt::Debug;

use crate::data::Fork;
use crate::Value;
use crate::Values;

pub fn new_small_stack() -> Values {
    Values::new()
}

pub fn new_large_stack() -> impl Stack {
    Vec::new()
}

pub trait Stack: Debug {

    fn push(&mut self, value: Value);

    fn push_all(&mut self, values: Values);

    fn pop(&mut self) -> Option<Value>;

    fn peek(&self) -> Option<Value>;

    fn size(&self) -> usize;

    fn as_debug_str(&self) -> String;
}

impl Stack for Vec<Value> {

    fn push(&mut self, value: Value) {
        self.push(value)
    }

    fn push_all(&mut self, values: Values) {
        self.extend(values)
    }

    fn pop(&mut self) -> Option<Value> {
        self.pop()
    }

    fn peek(&self) -> Option<Value> {
        self.last().map(|val| val.fork())
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn as_debug_str(&self) -> String {
        to_debug_str(self.iter())
    }
}

impl Stack for Values {

    fn push(&mut self, value: Value) {
        self.push(value)
    }

    fn push_all(&mut self, values: Values) {
        self.extend(values)
    }

    fn pop(&mut self) -> Option<Value> {
        self.pop()
    }

    fn peek(&self) -> Option<Value> {
        self.last().map(|val| val.fork())
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn as_debug_str(&self) -> String {
        to_debug_str(self.iter())
    }
}

fn to_debug_str<'a>(stack: impl Iterator<Item=&'a Value>) -> String {
    stack
        .map(|s| format!("{:?}", s))
        .collect::<Vec<_>>()
        .join(" | ")
}
