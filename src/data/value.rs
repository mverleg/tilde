use ::std::fmt;
use ::std::fmt::Formatter;

use ::tinyvec::TinyVec;

use crate::Array;
use crate::Nr;
use crate::Text;

#[derive(Debug, PartialEq)]
pub enum Value {
    Num(Nr),
    Txt(Text),
    Arr(Array),
    //TODO @mark: Func(),
}

pub type Values = TinyVec<[Value; 2]>;

impl fmt::Display for Value {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Value::Num(val) => write!(f, "{val}"),
            Value::Txt(val) => write!(f, "{val}"),
            Value::Arr(val) => write!(f, "{val}"),
        }
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::Txt(Text::from(val))
    }
}

impl From<Nr> for Value {
    fn from(val: Nr) -> Self {
        Value::Num(val)
    }
}

impl From<Text> for Value {
    fn from(val: Text) -> Self {
        Value::Txt(val)
    }
}

impl From<Array> for Value {
    fn from(val: Array) -> Self {
        Value::Arr(val)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Num(Nr::zero())
    }
}
