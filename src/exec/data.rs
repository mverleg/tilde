
#[derive(Debug)]
pub enum Value {
    Num(Number),
    Txt(Text),
    Arr(Array),
}


#[derive(Debug)]
pub struct Number {
    val: f64,
}

#[derive(Debug)]
pub struct Text {
    val: String,
}

#[derive(Debug)]
pub struct Array {
    val: Vec<Value>,
}

impl Array {
    pub fn of(vec: Vec<Value>) -> Self {
        Array { val: vec }
    }

    pub fn single(val: Value) -> Self {
        Array { val: vec![val] }
    }
}
