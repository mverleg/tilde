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

    pub fn single(modi: Token) -> Self {
        assert!(modi.is_modifier());
        Modifiers {
            first: Some(modi),
            second: None,
        }
    }

    pub fn double(first: Token, second: Token) -> TildeRes<Self> {
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
}
