use ::std::fmt;
use ::std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum Value {
    None,
    Num(Number),
    Txt(Text),
    Arr(Array),
}

impl fmt::Display for Value {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Value::None => Ok(()),
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

impl From<Number> for Value {
    fn from(val: Number) -> Self {
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

#[derive(Debug, PartialEq)]
pub struct Number {
    val: f64,
}

impl Number {
    pub fn of(val: f64) -> Self {
        Number { val }
    }

    pub fn value(&self) -> f64 {
        self.val
    }
}

impl fmt::Display for Number {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Text {
    val: String,
}

impl Text {
    pub fn of(val: impl Into<String>) -> Self {
        Text { val: val.into() }
    }
}

impl fmt::Display for Text {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl From<String> for Text {
    fn from(val: String) -> Self {
        Text::of(val)
    }
}

#[derive(Debug, PartialEq)]
pub struct Array {
    val: Vec<Value>,
}

impl fmt::Display for Array {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
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
            None => Value::None,
        }
    }

    pub fn get_mut(&mut self) -> &mut Vec<Value> {
        &mut self.val
    }
}
