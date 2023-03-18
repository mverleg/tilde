use ::std::fmt;
use ::std::fmt::Formatter;

use ::tinyvec::TinyVec;

use crate::Array;
use crate::Nr;
use crate::Text;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Num(Nr),
    Txt(Text),
    Arr(Array),
    Func(Func),
    //TODO @mark: Func(),
}

pub type Values = TinyVec<[Value; 2]>;

// impl Value {
//     pub fn num(nr: impl Into<Nr>) -> Value {
//         Value::Num(nr.into())
//     }
//     pub fn txt(text: impl Into<Text>) -> Value {
//         Value::Txt(text.into())
//     }
// }
//TODO @mark: TEMPORARY! REMOVE THIS!

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(val) => write!(f, "{val}"),
            Value::Txt(val) => write!(f, "{val}"),
            Value::Arr(val) => write!(f, "{val}"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(val) => write!(f, "{val}"),
            Value::Txt(val) => write!(f, "\"{val}\""),
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

impl From<u64> for Value {
    fn from(val: u64) -> Self {
        Value::Num(Nr::from(val))
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Num(Nr::from(val))
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

#[macro_export]
macro_rules! values {
    () => {{
        use ::tinyvec::TinyVec;
        let vs: Values = TinyVec::new();
        vs
    }};
    ($($vals:expr),+ $(,)?) => {{
        use ::tinyvec::TinyVec;
        use $crate::Value;
        let mut vs: Values = TinyVec::new();
        $({
            let v: Value = $vals.into();
            vs.push(v);
        })*
        vs
    }};
}

pub use values;
use crate::data::closure::Func;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_macro() {
        let v = values![];
        assert!(v.is_empty());
        let v = values![1, 2, 3];
        assert_eq!(v.len(), 3);
    }
}
