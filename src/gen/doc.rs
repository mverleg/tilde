use ::std::fmt;
use ::std::fmt::Formatter;

use crate::gen::input::gen_inputs;
use crate::parse::{Token, TokenGroup};
use crate::TildeRes;

#[derive(Debug)]
pub struct GroupDoc {
    token_group: TokenGroup,
}

impl fmt::Display for GroupDoc {
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
pub fn gen_grouped_docs() -> Vec<Vec<Token, GroupDoc>> {}

pub fn gen_docs() -> Vec<GroupDoc> {
    let mut docs = vec![];
    let mut buf = String::new();
    for token_group in gen_inputs() {
        docs.push(GroupDoc { token_group });
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
