//TODO @mverleg: for now, reject duplicate modifiers and enforce order - this way is can be relaxed later without breaking compatibility

use crate::op::Op;
use crate::op::Prog;
use crate::op::ValueOp;
use crate::tilde_log;
use crate::TildeRes;

// mod alphabet;
// mod letter;
mod parse;
// mod word;

pub fn parse(src: &str) -> TildeRes<Prog> {
    let mut ops = vec![];
    let mut tokens = src.chars().collect::<Vec<_>>();
    let mut buffer = String::new();
    while let Some(current) = tokens.pop() {
        if current == ',' {
            tilde_log!("string literal, long mode");
            buffer.clear();
            while let Some(token) = tokens.pop() {
                if token == ',' {
                    //TODO @mark: build a way to escape commas
                    break;
                }
                buffer.push(token)
            }
            let op = Op::Value(ValueOp::Text(buffer.clone()));
            ops.push(op)
        } else if current == '"' {
            tilde_log!("string literal, short (lookup) mode");
            todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
        } else {
            todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
        }
    }
    Ok(Prog::of(ops))
}
