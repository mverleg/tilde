//TODO @mverleg: for now, reject duplicate modifiers and enforce order - this way is can be relaxed later without breaking compatibility

use ::std::env::current_exe;

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
        if current == ',' || current == '\'' {
            buffer.clear();
            while let Some(token) = tokens.pop() {
                if token == ',' || token == '\'' {
                    //TODO @mark: build a way to escape commas
                    break;
                }
                buffer.push(token)
            }
            tilde_log!("string literal (long mode): '{}'", &buffer);
            let op = Op::Value(ValueOp::Text(buffer.clone()));
            ops.push(op)
        } else if (current >= '1' && current <= '9') || current == '.' || current == '-' {
            // note that short-mode numbers start with 0, long-mode ones cannot
            buffer.clear();
            buffer.push(current);
            while let Some(token) = tokens.pop() {
                if (token < '0' || token > '9') && token != '.' && current != '-' {
                    tokens.push(token);
                    break;
                }
                buffer.push(token)
            }
            tilde_log!("integer literal (long mode): \"{}\"", &buffer);
            let op = Op::Value(ValueOp::Number(buffer.parse::<f64>().map_err(|err| format!("invalid number '{}', err {}", buffer, err))?));
            ops.push(op)
        } else if current.is_alphabetic() || current == '-' {
            buffer.clear();
            buffer.push(current);
            while let Some(token) = tokens.pop() {
                if !current.is_alphabetic() && current != '-' {
                    if !current.is_whitespace() {
                        tokens.push(token);
                    }
                    break;
                }
                buffer.push(token)
            }
            tilde_log!("operator by long name: \"{}\"", &buffer);
            let op = todo!("{}", current);
            ops.push(op)
        } else if current.is_whitespace() {
            tilde_log!("skipping whitespace");
        } else if current == '"' {
            tilde_log!("string lookup (short mode)");
            todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
        } else {
            todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
        }
    }
    Ok(Prog::of(ops))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn of(op: Op) -> Prog {
        Prog::of(vec![op])
    }

    #[test]
    fn long_string_explicit_close() {
        assert_eq!(parse(",hello world,").unwrap(), of(Op::Value(ValueOp::Text("hello world".to_string()))));
        assert_eq!(parse(",hello world'").unwrap(), of(Op::Value(ValueOp::Text("hello world".to_string()))));
        assert_eq!(parse("'hello world,").unwrap(), of(Op::Value(ValueOp::Text("hello world".to_string()))));
        assert_eq!(parse("'hello world'").unwrap(), of(Op::Value(ValueOp::Text("hello world".to_string()))));
    }

    #[test]
    fn long_string_implicit_close() {
        assert_eq!(parse(",hello world").unwrap(), of(Op::Value(ValueOp::Text("hello world".to_string()))));
        assert_eq!(parse("'hello world").unwrap(), of(Op::Value(ValueOp::Text("hello world".to_string()))));
    }

    #[test]
    fn long_integer() {
        assert_eq!(parse("123").unwrap(), of(Op::Value(ValueOp::Number(123.))));
        assert_eq!(parse("-123").unwrap(), of(Op::Value(ValueOp::Number(-123.))));
    }

    #[test]
    fn long_float() {
        assert_eq!(parse("1.23").unwrap(), of(Op::Value(ValueOp::Number(1.23))));
        assert_eq!(parse(".123").unwrap(), of(Op::Value(ValueOp::Number(0.123))));
        assert_eq!(parse("123.").unwrap(), of(Op::Value(ValueOp::Number(123.))));
        assert_eq!(parse("-1.23").unwrap(), of(Op::Value(ValueOp::Number(-1.23))));
        assert_eq!(parse("-.123").unwrap(), of(Op::Value(ValueOp::Number(-0.123))));
        assert_eq!(parse("-123.").unwrap(), of(Op::Value(ValueOp::Number(-123.))));
    }

    #[test]
    fn long_invalid_number() {
        assert!(parse("1.2.3").is_err());
    }

    #[test]
    fn operator_by_name() {
        assert_eq!(parse("myOpName").unwrap(), of(todo!()));
        assert_eq!(parse("my-op-name").unwrap(), of(todo!()));
    }

    #[test]
    fn split_on_whitespace() {
        let op = Op::Value(ValueOp::Number(1.23));
        assert_eq!(parse("'hello' 1.0").unwrap(), Prog::of(vec![Op::Value(ValueOp::Text("hello".to_string())), Op::Value(ValueOp::Number(1.0))]));
        assert_eq!(
            parse("my-op-name my-op-name   1.0").unwrap(),
            Prog::of(vec![Op::Value(ValueOp::Text("hello".to_string())), Op::Value(ValueOp::Text("hello".to_string())), Op::Value(ValueOp::Number(1.0))])
        );
    }
}
