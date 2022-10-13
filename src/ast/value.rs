use crate::ast::typ::Typ;

#[derive(Debug)]
pub enum ValueOp {
    Number(f64),
    // TODO @mverleg: change to something exact
}

impl ValueOp {
    pub fn name(&self) -> &str {
        match self {
            ValueOp::Number(nr) => "0-9",
        }
    }

    pub fn description(&self, typ: Typ) -> &str {
        todo!();
    }
}
