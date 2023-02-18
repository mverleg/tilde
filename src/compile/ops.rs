use crate::op::Op;

/// Find [OpTyp] by identifeir. Not applicable for literals (text, number).
pub fn lookup_op_name(op_name: &str) -> Option<Op> {
    todo!();  //TODO @mark: TEMPORARY! REMOVE THIS!
    Some(match op_name {
        // "neg" => OpTyp::Neg,
        // "abs" => OpTyp::Abs,
        // "incr" => OpTyp::Incr,
        // "decr" => OpTyp::Decr,
        //
        // "plus" => OpTyp::Plus,
        // "minus" => OpTyp::Minus,
        // "mul" => OpTyp::Mul,
        // "div" => OpTyp::Div,
        // "int-div" => OpTyp::IntDiv,
        // "modulo" => OpTyp::Mod,
        //
        // "eq" => OpTyp::Eq,
        // "neq" => OpTyp::Neq,
        // "gt" => OpTyp::Gt,
        // "gte" => OpTyp::Gte,
        // "lt" => OpTyp::Lt,
        // "lte" => OpTyp::Lte,
        //
        // "and" => OpTyp::And,
        // "or" => OpTyp::Or,
        // "nand" => OpTyp::Nand,
        // "xor" => OpTyp::Xor,
        // "impl" => OpTyp::Impl,

        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use crate::op::all_non_literals;

    use super::*;

    #[test]
    fn all_ops_parseable() {
        for orig_op in all_non_literals() {
            let name = orig_op.name();
            if name == "text" || name == "nr" {
                continue;
            }
            let parse_op = lookup_op_name(name);
            assert!(parse_op.is_some(), "could not parse op: {name}, add it to `lookup_op_name`");
        }
    }
}
