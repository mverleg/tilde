use crate::op::typ::Typ;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueOp {
    Text(String),
    Number(f64),
    // TODO @mverleg: change to something exact
}

impl ValueOp {
    pub fn name(&self) -> &str {
        match self {
            ValueOp::Text(txt) => "text",
            ValueOp::Number(nr) => "0-9",
        }
    }

    pub fn description(
        &self,
        typ: Typ,
    ) -> &str {
        todo!();
    }
}
