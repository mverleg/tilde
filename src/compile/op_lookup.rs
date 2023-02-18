use ::std::cell::LazyCell;
use ::std::collections::HashMap;
use ::std::borrow::Cow;

use crate::compile::GolfWord;
use crate::op::{all_non_literals, Op};
use crate::tilde_log;

thread_local! {
    static GOLF_OP_LOOKUP: LazyCell<HashMap<GolfWord, Op>> = LazyCell::new(init_golf_op_lookup);
    static LONG_OP_LOOKUP: LazyCell<HashMap<&'static str, Op>> = LazyCell::new(init_long_op_lookup);
    //TODO @mark: fnv map
}

/// Find [OpTyp] by identifeir. Not applicable for literals (text, number).
pub fn lookup_op_long(op_name: &str) -> Option<Op> {
    LONG_OP_LOOKUP.with(|lookup| lookup.get(op_name).cloned())
}

//TODO @mark: use
fn init_golf_op_lookup() -> HashMap<GolfWord, Op> {
    tilde_log!("initializing lookup map by golf code");
    todo!()   //TODO @mark:
}

fn init_long_op_lookup() -> HashMap<&'static str, Op> {
    tilde_log!("initializing lookup map by long identifier");
    all_non_literals().into_iter()
        .map(|op| {
            let Cow::Borrowed(name) = op.long_code() else {
                unreachable!()
            };
            (name, op)
        })
        .collect::<HashMap<&'static str, Op>>()
}

#[cfg(test)]
mod tests {
    use crate::op::all_non_literals;

    use super::*;

    #[test]
    fn all_ops_parseable() {
        for orig_op in all_non_literals() {
            let name = orig_op.long_code();
            let parse_op = lookup_op_long(name.as_ref());
            assert!(parse_op.is_some(), "could not parse op: {name}, add it to `lookup_op_name`");
        }
    }
    //TODO @mark: similar to ^ for golf name
}
