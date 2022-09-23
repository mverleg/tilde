use crate::ast::{Math1Op, Op};

use crate::gen::input::gen_inputs;
use crate::parse::{Token, TokenGroup, TOKENSET};

#[derive(Debug)]
pub struct OpDoc {
    token_group: TokenGroup,
    operation: Op,
}

impl OpDoc {
    pub fn chars(&self) -> String {
        self.token_group.chars()
    }

    pub fn op_name(&self) -> &str {
        self.operation.name()
    }
}

/// Generate document objects, grouped by opener and sorted.
//TODO @mverleg: include standalone modifiers
pub fn gen_grouped_docs() -> Vec<(Token, Vec<OpDoc>)> {
    let mut groups = TOKENSET
        .iter()
        .map(|token| (token.clone(), vec![]))
        .collect::<Vec<(Token, Vec<OpDoc>)>>();
    for op_doc in gen_docs() {
        let token_index = op_doc.token_group.group().byte as usize;
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
        docs.push(OpDoc {
            token_group,
            operation: Op::Math1(Math1Op::Incr),
        });
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
