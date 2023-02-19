use ::std::fmt;

use crate::data::value::Value;

#[derive(Debug, PartialEq)]
pub struct Array {
    val: Vec<Value>,
}

impl fmt::Display for Array {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for item in &self.val {
            write!(f, "{item}")?
        }
        Ok(())
    }
}

impl Array {
    pub fn of<V: Into<Value>>(vec: Vec<V>) -> Self {
        Array {
            val: vec
                .into_iter()
                .map(|v| v.into())
                .collect(),
        }
    }

    pub fn single<V: Into<Value>>(val: V) -> Self {
        Array::of(vec![val])
    }

    pub fn push(
        &mut self,
        val: Value,
    ) {
        self.val.push(val)
    }

    pub fn pop(&mut self) -> Value {
        match self.val.pop() {
            Some(val) => val,
            None => Value::default(),
        }
    }

    pub fn get_mut(&mut self) -> &mut Vec<Value> {
        &mut self.val
    }
}
