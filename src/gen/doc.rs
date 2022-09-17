use ::std::fmt;
use ::std::fmt::Formatter;

use crate::gen::input::gen_inputs;
use crate::parse::{Token, TokenGroup, TOKENSET};

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
    let mut groups = TOKENSET
        .iter()
        .map(|token| (token.clone(), vec![]))
        .collect::<Vec<(Token, Vec<OpDoc>)>>();
    dbg!(groups.len()); //TODO @mverleg: TEMPORARY! REMOVE THIS!
    for op_doc in gen_docs() {
        let token_index = op_doc.token_group.group().byte as usize;
        dbg!(token_index); //TODO @mverleg: TEMPORARY! REMOVE THIS!
        groups[token_index].1.push(op_doc);
    }
    // gen_docs()
    //     .group_by(|token| token.group())
    //     .collect();
    groups
}

pub fn gen_docs() -> Vec<OpDoc> {
    let mut docs = vec![];
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
