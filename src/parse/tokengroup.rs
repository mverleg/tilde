use crate::parse::Token;
use crate::TildeRes;

//TODO @mverleg: this is only suitable for general context for now

#[derive(Debug, Clone)]
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
            return Err(format!(
                "if {first} and {second} appear together, {first} must be first"
            ));
        }
        if first.byte < second.byte {
            return Err(format!(
                "if {first} and {second} appear together, {first} must be first"
            ));
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
