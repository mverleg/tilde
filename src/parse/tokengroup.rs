use ::std::mem;

use crate::ast::Math2Op::Mod;
use crate::parse::token::TokenType;
use crate::parse::Token;
use crate::TildeRes;

//TODO @mverleg: this is only suitable for general context for now

#[derive(Debug)]
pub struct Modifiers {
    first: Option<Token>,
    second: Option<Token>,
}

impl Modifiers {
    pub fn empty() -> Self {
        Modifiers {
            first: None,
            second: None,
        }
    }

    pub fn single(modi: Token) -> Self {
        assert!(modi.is_modifier());
        Modifiers {
            first: Some(modi),
            second: None,
        }
    }

    pub fn double(mut first: Token, mut second: Token) -> TildeRes<Self> {
        assert!(first.is_modifier());
        assert!(second.is_modifier());
        if first == second {
            Err(format!(
                "if {first} and {second} appear together, {first} must be first"
            ))
        }
        if first < second {
            Err(format!(
                "if {first} and {second} appear together, {first} must be first"
            ))
        }
        Ok(Modifiers {
            first: Some(first),
            second: Some(second),
        })
    }
}

#[derive(Debug)]
pub enum TokenGroup {
    Var(Token, Modifiers),
    Fixed(Token, Token, Modifiers),
    JustMod(Modifiers),
}
