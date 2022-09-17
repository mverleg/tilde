use ::std::fmt;
use ::std::fmt::Formatter;

use crate::gen::input::gen_inputs;
use crate::parse::{Modifiers, Token, TokenGroup};

#[derive(Debug)]
pub struct OpDoc {
    token_group: TokenGroup,
}

impl fmt::Display for OpDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "input: ")?;
        self.token_group.fmt_chars(f)?;
        write!(f, " ; bytes: ")?;
        self.token_group.fmt_bytes(f)

        //TODO @mverleg: parse the group and if successful, print description
    }
}

/// Generate document objects, grouped by opener and sorted.
//TODO @mverleg: include standalone modifiers
pub fn gen_grouped_docs() -> Vec<(Token, Vec<OpDoc>)> {
    let mut groups = vec![];
    // gen_docs()
    //     .group_by(|token| token.group())
    //     .collect();
    groups
}

pub fn gen_docs() -> Vec<OpDoc> {
    let mut docs = vec![];
    let mut buf = String::new();
    for token_group in gen_inputs() {
        docs.push(OpDoc { token_group });
    }
    docs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_docs() {
        for doc in gen_docs() {
            println!("{}", doc);
        }
    }
}
