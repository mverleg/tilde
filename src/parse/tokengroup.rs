use ::std::fmt;

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
    fn fmt_chars(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(m) = &self.first {
            write!(f, "{}", m.chr)?;
            if let Some(m) = &self.second {
                write!(f, "{}", m.chr)?
            }
        }
        Ok(())
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

#[derive(Debug)]
pub enum TokenGroup {
    Var(Token, Modifiers),
    Fixed(Token, Token, Modifiers),
    JustMod(Modifiers),
}

impl TokenGroup {
    pub(crate) fn fmt_chars(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenGroup::Var(open, modi) => {
                write!(f, "{}", open.chr)?;
                modi.fmt_chars(f)
            }
            TokenGroup::Fixed(open, second, modi) => {
                write!(f, "{}{}", open.chr, second.chr)?;
                modi.fmt_chars(f)
            }
            TokenGroup::JustMod(modi) => modi.fmt_chars(f),
        }
    }

    pub(crate) fn fmt_bytes(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenGroup::Var(open, modi) => {
                write!(f, "{:x}", open.byte)?;
                modi.fmt_bytes(f)
            }
            TokenGroup::Fixed(open, second, modi) => {
                write!(f, "{:x},{:x}", open.byte, second.byte)?;
                modi.fmt_bytes(f)
            }
            TokenGroup::JustMod(modi) => modi.fmt_bytes(f),
        }
    }

    pub fn group(&self) -> &Token {
        match &self {
            TokenGroup::Var(opener, _) => opener,
            TokenGroup::Fixed(opener, _, _) => opener,
            TokenGroup::JustMod(modi) => modi.first().as_ref().unwrap(),
        }
    }

    pub fn order(&self) -> u32 {
        match &self {
            TokenGroup::Var(opener, modifiers) => {
                Self::calc_order_for(Some(opener), None, modifiers)
            }
            TokenGroup::Fixed(opener, second, modifiers) => {
                Self::calc_order_for(Some(opener), Some(second), modifiers)
            }
            TokenGroup::JustMod(modifiers) => Self::calc_order_for(None, None, modifiers),
        }
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
