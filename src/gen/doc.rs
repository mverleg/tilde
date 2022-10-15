use crate::ast::{Math1Op, Op};

use crate::compile::{Letter, Word, ALPHABET};
use crate::gen::input::gen_inputs;

#[derive(Debug)]
pub struct OpDoc {
    word: Word,
    operation: Op,
}

impl OpDoc {
    pub fn chars(&self) -> String {
        self.word.chars()
    }

    pub fn op_name(&self) -> &str {
        self.operation.name()
    }
}

/// Generate document objects, grouped by opener and sorted.
//TODO @mverleg: include standalone modifiers
pub fn gen_grouped_docs() -> Vec<(Letter, Vec<OpDoc>)> {
    let mut groups = ALPHABET
        .iter()
        .map(|letter| (letter.clone(), vec![]))
        .collect::<Vec<(Letter, Vec<OpDoc>)>>();
    for op_doc in gen_docs() {
        let index = op_doc.word.group().byte as usize;
        groups[index].1.push(op_doc);
    }
    // gen_docs()
    //     .group_by(|token| token.group())
    //     .collect();
    groups
}

pub fn gen_docs() -> Vec<OpDoc> {
    let mut docs = vec![];
    for word in gen_inputs() {
        docs.push(OpDoc {
            word,
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
            println!("{:?}", doc);
        }
        //TODO @mark:
    }
}
