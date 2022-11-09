use ::std::env::current_exe;

use crate::compile::ops::lookup_op_name;
use crate::op::Op;
use crate::op::Prog;
use crate::tilde_log;
use crate::TildeRes;

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
            let op = Op::Text(buffer.clone());
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
            let op = Op::Number(buffer.parse::<f64>().map_err(|err| format!("invalid number '{}', err {}", buffer, err))?);
            ops.push(op)
        } else if current.is_alphabetic() || current == '-' {
            buffer.clear();
            buffer.push(current);
            while let Some(token) = tokens.pop() {
                if !token.is_alphabetic() && token != '-' {
                    if !current.is_whitespace() {
                        tokens.push(token);
                    }
                    break;
                }
                buffer.push(token)
            }
            tilde_log!("operator by long name: \"{}\"", &buffer);
            let op = lookup_op_name(&buffer).ok_or_else(|| format!("could not find an identifier by name '{}'", &buffer))?;
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
        assert_eq!(parse(",hello world,").unwrap(), of(Op::Text("hello world".to_string())));
        assert_eq!(parse(",hello world'").unwrap(), of(Op::Text("hello world".to_string())));
        assert_eq!(parse("'hello world,").unwrap(), of(Op::Text("hello world".to_string())));
        assert_eq!(parse("'hello world'").unwrap(), of(Op::Text("hello world".to_string())));
    }

    #[test]
    fn long_string_implicit_close() {
        assert_eq!(parse(",hello world").unwrap(), of(Op::Text("hello world".to_string())));
        assert_eq!(parse("'hello world").unwrap(), of(Op::Text("hello world".to_string())));
    }

    #[test]
    fn long_integer() {
        assert_eq!(parse("123").unwrap(), of(Op::Number(123.)));
        assert_eq!(parse("-123").unwrap(), of(Op::Number(-123.)));
    }

    #[test]
    fn long_float() {
        assert_eq!(parse("1.23").unwrap(), of(Op::Number(1.23)));
        assert_eq!(parse(".123").unwrap(), of(Op::Number(0.123)));
        assert_eq!(parse("123.").unwrap(), of(Op::Number(123.)));
        assert_eq!(parse("-1.23").unwrap(), of(Op::Number(-1.23)));
        assert_eq!(parse("-.123").unwrap(), of(Op::Number(-0.123)));
        assert_eq!(parse("-123.").unwrap(), of(Op::Number(-123.)));
    }

    #[test]
    fn long_invalid_number() {
        assert!(parse("1.2.3").is_err());
    }

    #[test]
    fn operator_by_name() {
        assert_eq!(parse("div").unwrap(), of(Op::Div));
        assert_eq!(parse("int-div").unwrap(), of(Op::IntDiv));
    }

    #[test]
    fn unknown_operator_by_name() {
        assert!(parse("unknownOperator").is_err());
        assert!(parse("unknown-operator").is_err());
    }

    #[test]
    fn split_on_whitespace() {
        let op = Op::Number(1.23);
        assert_eq!(parse("'hello' 1.0").unwrap(), Prog::of(vec![Op::Text("hello".to_string()), Op::Number(1.0)]));
        assert_eq!(parse("div   1").unwrap(), Prog::of(vec![Op::Div, Op::Number(1.0)]),);
        assert_eq!(parse("int-div1.0").unwrap(), Prog::of(vec![Op::IntDiv, Op::Number(1.0)]),);
    }
}
