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
    tokens.reverse();
    tilde_log!("parsing {} tokens", tokens.len());
    let mut buffer = String::new();
    while let Some(current) = tokens.pop() {
        if current == ',' {
            buffer.clear();
            while let Some(token) = tokens.pop() {
                if token == ',' {
                    //TODO @mark: build a way to escape commas
                    break;
                }
                buffer.push(token)
            }
            tilde_log!("string literal (long mode): \"{}\"", &buffer);
            let op = Op::Value(ValueOp::Text(buffer.clone()));
            ops.push(op)
        } else if (current >= '1' && current <= '9') || current == '.' {
            // note that short-mode numbers start with 0, long-mode ones cannot
            buffer.clear();
            while let Some(token) = tokens.pop() {
                if (token < '0' || token > '9') && token != '.' {
                    tokens.push(token);
                    break;
                }
                buffer.push(token)
            }
            tilde_log!("integer literal (long mode): \"{}\"", &buffer);
            let op = Op::Value(ValueOp::Text(buffer.clone()));
            ops.push(op)
        } else if current == '"' {
            tilde_log!("string lookup (short mode)");
            todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
        } else {
            todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
        }
    }
    Ok(Prog::of(ops))
}
