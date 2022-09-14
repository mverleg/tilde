
#[derive(Debug)]
pub enum Value {
    Num(Number),
    Txt(Text),
    Arr(Array),
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

#[derive(Debug)]
pub struct Number {
    val: f64,
}

#[derive(Debug)]
pub struct Text {
    val: String,
}

impl Text {
    pub fn of(val: impl Into<String>) -> Self {
        Text { val: val.into() }
    }
}

impl From<String> for Text {
    fn from(val: String) -> Self {
        Text::of(val)
    }
}

#[derive(Debug)]
pub struct Array {
    val: Vec<Value>,
}

impl Array {
    pub fn of<V: Into<Value>>(vec: Vec<V>) -> Self {
        Array {
            val: vec.into_iter().map(|v| v.into()).collect()
        }
    }

    pub fn single<V: Into<Value>>(val: V) -> Self {
        Array::of(vec![val])
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.val.pop()
    }
}
