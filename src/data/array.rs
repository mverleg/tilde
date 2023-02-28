use ::std::fmt;

use crate::data::value::Value;

#[derive(PartialEq, Clone)]
pub struct Array {
    val: Vec<Value>,
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut is_first = true;
        for item in &self.val {
            if is_first {
                is_first = false;
            } else {
                write!(f, ",")?;
            }
            write!(f, "{item}")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut is_first = true;
        for item in &self.val {
            if is_first {
                is_first = false;
            } else {
                write!(f, ",")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")?;
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

#[cfg(test)]
mod tests {
    use ::std::f64::consts::PI;

    use crate::Nr;
    use crate::Text;

    use super::*;

    #[test]
    fn fmt_array_single() {
        let arr = Array::single(Value::Num(Nr::new(PI)));
        assert_eq!(format!("{}", arr), "[3.14]")
    }

    #[test]
    fn debug_array_single() {
        let arr = Array::single(Value::Num(Nr::new(PI)));
        assert_eq!(format!("{:?}", arr), "[3.14]")
    }

    #[test]
    fn fmt_array_multi() {
        let arr = Array::of(vec![Value::Num(Nr::new(PI)), Value::Txt(Text::of("hello"))]);
        assert_eq!(format!("{}", arr), "[3.14,hello]")
    }

    #[test]
    fn debug_array_multi() {
        let arr = Array::of(vec![Value::Num(Nr::new(PI)), Value::Txt(Text::of("hello"))]);
        assert_eq!(format!("{:?}", arr), "[3.14,\"hello\"]")
    }
}
