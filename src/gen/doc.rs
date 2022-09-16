use ::std::fmt;
use ::std::fmt::Formatter;

use crate::gen::input::gen_inputs;
use crate::parse::TokenGroup;

#[derive(Debug)]
pub struct GroupDoc {
    token_group: TokenGroup,
}

impl fmt::Display for GroupDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "input: ")?;
        self.token_group.fmt_chars(f)?;
        write!(f, ", bytes: ")?;
        self.token_group.fmt_bytes(f)

        //TODO @mverleg: parse the group and if successful, print description
    }
}

pub fn gen_docs() {
    for token_group in gen_inputs() {
        GroupDoc { token_group };
    }
    todo!(); //TODO @mverleg: TEMPORARY! REMOVE THIS!
}
