use crate::ast::Prog;
use crate::exec::data::Array;

mod data;

pub fn execute(prog: Prog, mut inp: Vec<String>) {
    inp.reverse();
    let stack = Array::single(Array::of(inp));
}
