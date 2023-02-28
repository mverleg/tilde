use ::std::fmt::Debug;

use crate::{Nr, tilde_log};
use crate::compile::{GolfWord, Letter, LetterKind};
use crate::compile::golf_word::GolfWordContent;
use crate::compile::op_lookup::{lookup_op_golf, lookup_op_long};
use crate::compile::Prog;
use crate::compile::text_literal::decode_str;
use crate::op::{NumberOp, TextOp};
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
            let op = TextOp::new(string_buffer.clone());
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
            let op = NumberOp::new(
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
            let op = lookup_op_long(&string_buffer).ok_or_else(|| {
                tilde_log!("did not find long '{}', make sure it is in `all_non_literals`", &string_buffer);
                format!("could not find an identifier by name '{}'", &string_buffer)
            })?;
            ops.push(op)
        } else if let Some(golf_letter) = Letter::from_symbol(current) {
            let mut word = GolfWordContent::new();
            word.push(golf_letter);
            match golf_letter.kind() {
                LetterKind::Literal => unreachable!(),
                LetterKind::VariableOpen => {},
                LetterKind::FixedOpen => match rev_tokens.pop().map(|c| Letter::from_symbol(c)) {
                    Some(Some(second)) => word.push(second),
                    Some(None) => todo!("handle golf fixed opener expected next to be golf letter"),  //TODO @mark:
                    None => todo!("handle unexpected end"),  //TODO @mark:
                },
                LetterKind::Modifier => unimplemented!("cannot start with modifier"),
            }
            while let Some(chr) = rev_tokens.pop() {
                if let Some(letter) = Letter::from_symbol(chr) {
                    if letter.kind() == LetterKind::Modifier {
                        word.push(letter);
                        continue
                    }
                }
                rev_tokens.push(chr);
                break
            }
            tilde_log!("operator by short name: \"{}\"", &word);
            let op = lookup_op_golf(&GolfWord::new(word.clone())).ok_or_else(|| {
                //TODO @mark: get rid of clone? ^
                tilde_log!("did not find short '{}', make sure it is in `all_non_literals`", &word);
                format!("could not find golf code '{}'", &word)
            })?;
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
            ops.push(TextOp::new(string_buffer.clone()))
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
    use crate::op::Div;
    use crate::op::Drop;
    use crate::op::Duplicate;
    use crate::op::IntDiv;
    use crate::op::Op;

    use super::*;

    fn of(op: Op) -> Prog {
        Prog::of(vec![op])
    }

    #[test]
    fn long_string_explicit_close() {
        assert_eq!(parse(",hello world,").unwrap(), of(TextOp::new("hello world".to_string())));
        assert_eq!(parse(",hello world'").unwrap(), of(TextOp::new("hello world".to_string())));
        assert_eq!(parse("'hello world,").unwrap(), of(TextOp::new("hello world".to_string())));
        assert_eq!(parse("'hello world'").unwrap(), of(TextOp::new("hello world".to_string())));
    }

    #[test]
    fn long_string_implicit_close() {
        assert_eq!(parse(",hello world").unwrap(), of(TextOp::new("hello world".to_string())));
        assert_eq!(parse("'hello world").unwrap(), of(TextOp::new("hello world".to_string())));
    }

    #[test]
    fn golfed_string_explicit_close() {
        assert_eq!(parse("\"+>:[\"").unwrap(), of(TextOp::new("Hello world".to_string())));
        //assert_eq!(parse("\"+>:[0").unwrap(), of(TextOp::new("Hello world".to_string())));
        // maybe supported later? ^
    }

    #[test]
    fn golfed_string_implicit_close() {
        assert_eq!(parse("\"+>:[").unwrap(), of(TextOp::new("Hello world")));
    }

    #[test]
    fn golfed_string_multiple() {
        let expected = Prog::of(vec![TextOp::new("Hello world"), TextOp::new("Hello world")]);
        assert_eq!(parse("\"+>:[\"\"+>:[").unwrap(), expected);
    }

    #[test]
    fn long_integer() {
        assert_eq!(parse("123").unwrap(), of(NumberOp::new(123)));
        assert_eq!(parse("-123").unwrap(), of(NumberOp::new(-123)));
    }

    #[test]
    fn long_float() {
        assert_eq!(parse("1.23").unwrap(), of(NumberOp::new(1.23)));
        assert_eq!(parse(".123").unwrap(), of(NumberOp::new(0.123)));
        assert_eq!(parse("123.").unwrap(), of(NumberOp::new(123.)));
        assert_eq!(parse("-1.23").unwrap(), of(NumberOp::new(-1.23)));
        assert_eq!(parse("-.123").unwrap(), of(NumberOp::new(-0.123)));
        assert_eq!(parse("-123.").unwrap(), of(NumberOp::new(-123.)));
    }

    #[test]
    fn long_invalid_number() {
        assert!(parse("1.2.3").is_err());
    }

    #[test]
    fn operator_by_name() {
        assert_eq!(parse("div").unwrap(), of(Div::new()));
        assert_eq!(parse("int-div").unwrap(), of(IntDiv::new()));
    }

    #[test]
    fn unknown_operator_by_name() {
        assert!(parse("unknownOperator").is_err());
        assert!(parse("unknown-operator").is_err());
    }

    #[test]
    fn operator_by_gold() {
        assert_eq!(parse("/").unwrap(), of(Drop::new()));
        assert_eq!(parse("+").unwrap(), of(Duplicate::new()));
    }

    #[test]
    fn split_on_whitespace() {
        let op = NumberOp::new(1.23);
        assert_eq!(parse("'hello' 1.0").unwrap(), Prog::of(vec![TextOp::new("hello"), NumberOp::new(1.0)]));
        assert_eq!(parse("div   1").unwrap(), Prog::of(vec![Div::new(), NumberOp::new(1.0)]),);
        assert_eq!(parse("int-div1.0").unwrap(), Prog::of(vec![IntDiv::new(), NumberOp::new(1.0)]),);
    }

    #[test]
    fn allow_newlines() {
        let op = NumberOp::new(1.23);
        assert_eq!(parse("'hello'\n1.0").unwrap(), Prog::of(vec![TextOp::new("hello"), NumberOp::new(1.0)]));
        assert_eq!(parse("div\n1").unwrap(), Prog::of(vec![Div::new(), NumberOp::new(1.0)]),);
    }

    //TODO @mark: add some golfed testcases to existing tests, like whitespace ones
}
