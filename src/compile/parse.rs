use ::std::env::current_exe;
use ::std::fmt::Debug;

use crate::compile::Letter;
use crate::compile::ops::lookup_op_name;
use crate::compile::Prog;
use crate::compile::text_literal::decode_str;
use crate::op::Op;
use crate::{Nr, tilde_log};
use crate::TildeRes;

pub fn parse(src: &str) -> TildeRes<Prog> {
    let mut ops = vec![];
    let mut rev_tokens = src
        .chars()
        .collect::<Vec<_>>();
    rev_tokens.reverse();
    tilde_log!("parsing {} tokens", rev_tokens.len());
    let mut string_buffer = String::new();
    let mut letters_buffer = Vec::new();
    let mut string_decode_buffer = Vec::new();
    while let Some(current) = rev_tokens.pop() {
        eprintln!("current = {current}");  //TODO @mark: TEMPORARY! REMOVE THIS!
        if current.is_whitespace() {
            tilde_log!("skipping whitespace");
        } else if current == ',' || current == '\'' {
            string_buffer.clear();
            while let Some(token) = rev_tokens.pop() {
                if token == ',' || token == '\'' {
                    //TODO @mark: build a way to escape commas
                    break;
                }
                string_buffer.push(token)
            }
            tilde_log!("string literal (long mode): '{}'", &string_buffer);
            let op = Op::Text(string_buffer.clone());
            ops.push(op)
        } else if ('1'..='9').contains(&current) || current == '.' || current == '-' {
            // note that short-mode numbers start with 0, long-mode ones cannot
            string_buffer.clear();
            string_buffer.push(current);
            while let Some(token) = rev_tokens.pop() {
                if !token.is_ascii_digit() && token != '.' && current != '-' {
                    rev_tokens.push(token);
                    break;
                }
                string_buffer.push(token)
            }
            tilde_log!("integer literal (long mode): \"{}\"", &string_buffer);
            let op = Op::Number(
                string_buffer
                    .parse::<Nr>()
                    .map_err(|err| format!("invalid number '{string_buffer}', err {err}"))?,
            );
            ops.push(op)
        } else if current.is_alphabetic() || current == '-' {
            string_buffer.clear();
            string_buffer.push(current);
            while let Some(token) = rev_tokens.pop() {
                if !token.is_alphabetic() && token != '-' {
                    if !current.is_whitespace() {
                        rev_tokens.push(token);
                    }
                    break;
                }
                string_buffer.push(token)
            }
            tilde_log!("operator by long name: \"{}\"", &string_buffer);
            let op = lookup_op_name(&string_buffer).ok_or_else(|| format!("could not find an identifier by name '{}'", &string_buffer))?;
            ops.push(op)
        } else if current == '"' {
            //TODO @mark: make more resistant to closer changes?
            letters_buffer.clear();
            while let Some(letter) = rev_tokens.pop().and_then(Letter::from_symbol) {
                letters_buffer.push(letter);
                if letter == Letter::Text || letter == Letter::Number {
                    break
                }
            }
            tilde_log!("string lookup (short mode), {} golf letters", letters_buffer.len());
            string_buffer.clear();
            let str_res = decode_str(&letters_buffer, &mut string_buffer, &mut string_decode_buffer)
                .map_err(|err| format!("could not parse golfed string, err: {err}"))?;
            ops.push(Op::Text(string_buffer.clone()))
        } else if let Some(golf_letter) = Letter::from_symbol(current) {
            todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
        } else {
            return Err(format!("unrecognized: {current}"))
        }
    }
    Ok(Prog::of(ops))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pos<T: Debug + PartialEq> {
    pub value: T,
    pub length: usize,
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
    fn golfed_string_explicit_close() {
        assert_eq!(parse("\"+>:[\"").unwrap(), of(Op::Text("Hello world".to_string())));
        //assert_eq!(parse("\"+>:[0").unwrap(), of(Op::Text("Hello world".to_string())));
        // maybe supported later? ^
    }

    #[test]
    fn golfed_string_implicit_close() {
        assert_eq!(parse("\"+>:[").unwrap(), of(Op::Text("Hello world".to_string())));
    }

    #[test]
    fn golfed_string_multiple() {
        let expected = Prog::of(vec![Op::Text("Hello world".to_string()), Op::Text("Hello world".to_string())]);
        assert_eq!(parse("\"+>:[\"\"+>:[").unwrap(), expected);
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

    #[test]
    fn allow_newlines() {
        let op = Op::Number(1.23);
        assert_eq!(parse("'hello'\n1.0").unwrap(), Prog::of(vec![Op::Text("hello".to_string()), Op::Number(1.0)]));
        assert_eq!(parse("div\n1").unwrap(), Prog::of(vec![Op::Div, Op::Number(1.0)]),);
    }

    //TODO @mark: add some golfed testcases to existing tests, like whitespace ones
}
