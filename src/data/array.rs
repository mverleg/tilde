use ::std::fmt;
use ::std::vec::IntoIter;

use crate::data::value::Value;
use crate::Nr;

#[derive(PartialEq, Eq, Hash, Clone)]
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

    pub fn index(&self, nr: Nr) -> Value {
        let ix: usize = nr.floor().try_into().expect("value to large to use as index");
        match self.val.get(ix) {
            Some(val) => {
                val.clone()
            },
            None => Value::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.val.len()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> IntoIter<Value> {
        self.val.into_iter()
    }
}

#[allow(clippy::approx_constant)]
#[cfg(test)]
mod tests {
    use crate::Nr;
    use crate::Text;

    use super::*;

    #[test]
    fn fmt_array_single() {
        let arr = Array::single(Value::Num(Nr::new(2.718)));
        assert_eq!(format!("{}", arr), "[2.718]")
    }

    #[test]
    fn debug_array_single() {
        let arr = Array::single(Value::Num(Nr::new(2.718)));
        assert_eq!(format!("{:?}", arr), "[2.718]")
    }

    #[test]
    fn fmt_array_multi() {
        let arr = Array::of(vec![Value::Num(Nr::new(2.718)), Value::Txt(Text::of("hello"))]);
        assert_eq!(format!("{}", arr), "[2.718,hello]")
    }

    #[test]
    fn debug_array_multi() {
        let arr = Array::of(vec![Value::Num(Nr::new(2.718)), Value::Txt(Text::of("hello"))]);
        assert_eq!(format!("{:?}", arr), "[2.718,\"hello\"]")
    }
}
