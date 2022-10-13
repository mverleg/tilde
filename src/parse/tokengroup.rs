use ::std::fmt;

use crate::parse::Token;
use crate::{TildeRes, NR};

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

    pub fn of_single(modi: Token) -> Self {
        assert!(modi.is_modifier());
        Modifiers {
            first: Some(modi),
            second: None,
        }
    }

    pub fn of_double(first: Token, second: Token) -> TildeRes<Self> {
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

    pub fn first(&self) -> &Option<Token> {
        &self.first
    }

    pub fn second(&self) -> &Option<Token> {
        &self.second
    }
}

impl Modifiers {
    fn chars(&self) -> String {
        if let Some(m) = &self.first {
            if let Some(n) = &self.second {
                format!("{}{}", m.chr, n.chr)
            } else {
                format!("{}", m.chr)
            }
        } else {
            "".to_string()
        }
    }

    /// Byte numbers with leading commas
    fn fmt_bytes(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(m) = &self.first {
            write!(f, ",{:x}", m.byte)?;
            if let Some(m) = &self.second {
                write!(f, ",{:x}", m.byte)?
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum TokenGroup {
    Text(String),
    Number(NR),
    Var(Token, Modifiers),
    Fixed(Token, Token, Modifiers),
    JustMod(Modifiers),
}

impl TokenGroup {
    pub(crate) fn chars(&self) -> String {
        match self {
            TokenGroup::Text(_txt) => todo!(),
            TokenGroup::Number(_nr) => todo!(),
            TokenGroup::Var(open, modi) => format!("{}{}", open.chr, modi.chars()),
            TokenGroup::Fixed(open, second, modi) => {
                format!("{}{}{}", open.chr, second.chr, modi.chars())
            }
            TokenGroup::JustMod(modi) => format!("{}", modi.chars()),
        }
    }

    // pub(crate) fn fmt_bytes(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     match self {
    //         TokenGroup::Var(open, modi) => {
    //             write!(f, "{:x}", open.byte)?;
    //             modi.fmt_bytes(f)
    //         }
    //         TokenGroup::Fixed(open, second, modi) => {
    //             write!(f, "{:x},{:x}", open.byte, second.byte)?;
    //             modi.fmt_bytes(f)
    //         }
    //         TokenGroup::JustMod(modi) => modi.fmt_bytes(f),
    //     }
    // }

    pub fn group(&self) -> &Token {
        todo!()
        // match &self {
        //     TokenGroup::Text(_) => Token::literal(),
        //     TokenGroup::Number(_) => todo!(),
        //     TokenGroup::Var(opener, _) => opener,
        //     TokenGroup::Fixed(opener, _, _) => opener,
        //     TokenGroup::JustMod(modi) => modi
        //         .first()
        //         .as_ref()
        //         .expect("opening modifiers should not be empty"),
        // }
    }

    pub fn order(&self) -> u32 {
        todo!()
        // match &self {
        //     TokenGroup::Var(opener, modifiers) => {
        //         Self::calc_order_for(Some(opener), None, modifiers)
        //     }
        //     TokenGroup::Fixed(opener, second, modifiers) => {
        //         Self::calc_order_for(Some(opener), Some(second), modifiers)
        //     }
        //     TokenGroup::JustMod(modifiers) => Self::calc_order_for(None, None, modifiers),
        // }
    }

    fn calc_order_for(first: Option<&Token>, second: Option<&Token>, modi: &Modifiers) -> u32 {
        ((Self::calc_token_value(first) * 257 + Self::calc_token_value(second)) * 257
            + Self::calc_token_value(modi.first().as_ref()))
            * 257
            + Self::calc_token_value(modi.second().as_ref())
    }

    fn calc_token_value(token: Option<&Token>) -> u32 {
        token.map(|t| t.byte + 1).unwrap_or(0) as u32
    }
}
