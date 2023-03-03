use ::std::borrow::Cow;
use ::std::cell::LazyCell;

use ::fnv::FnvHashMap;

use crate::compile::GolfWord;
use crate::op::{all_non_literals, Op};
use crate::tilde_log;

thread_local! {
    static LONG_OP_LOOKUP: LazyCell<FnvHashMap<&'static str, Op>> = LazyCell::new(init_long_op_lookup);
    static GOLF_OP_LOOKUP: LazyCell<FnvHashMap<GolfWord, Op>> = LazyCell::new(init_golf_op_lookup);
    //TODO @mark: fnv map
}

/// Find [Op] by identifier. Not applicable for literals (text, number).
pub fn lookup_op_long(op_name: &str) -> Option<Op> {
    LONG_OP_LOOKUP.with(|lookup| lookup.get(op_name).cloned())
}

/// Find [Op] by golf code, if it has a golf code (experimental ops may not). Not applicable for literals (text, number).
pub fn lookup_op_golf(op_name: &GolfWord) -> Option<Op> {
    GOLF_OP_LOOKUP.with(|lookup| lookup.get(op_name).cloned())
}

fn init_golf_op_lookup() -> FnvHashMap<GolfWord, Op> {
    tilde_log!("initializing lookup map by golf code");
    all_non_literals().into_iter()
        .flat_map(|op| op.golf_code().map(|name| (name, op)).into_iter())
        .collect()
}

fn init_long_op_lookup() -> FnvHashMap<&'static str, Op> {
    tilde_log!("initializing lookup map by long identifier");
    all_non_literals().into_iter()
        .map(|op| {
            let Cow::Borrowed(name) = op.long_code() else {
                unreachable!()
            };
            (name, op)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::op::all_non_literals;

    use super::*;

    #[test]
    fn all_long_ops_parseable() {
        for orig_op in all_non_literals() {
            let name = orig_op.long_code();
            let parse_op = lookup_op_long(name.as_ref());
            assert!(parse_op.is_some(), "could not parse op by long name: {name}");
        }
    }

    #[test]
    fn all_golf_ops_parseable() {
        for orig_op in all_non_literals() {
            let Some(name) = orig_op.golf_code() else {
                continue
            };
            let parse_op = lookup_op_golf(&name);
            assert!(parse_op.is_some(), "could not parse op by golf name: {name:?}");
        }
    }
    //TODO @mark: similar to ^ for golf name
}
