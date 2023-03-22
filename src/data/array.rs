use ::std::fmt;
use ::std::rc::Rc;
use ::std::vec;
use std::slice::Iter;

use crate::data::fork::Fork;
use crate::data::value::Value;
use crate::Nr;

#[derive(PartialEq, Eq, Hash)]
pub struct Array {
    val: Rc<Vec<Value>>,
}


impl Array {
    pub fn of<V: Into<Value>>(vec: Vec<V>) -> Self {
        Array {
            val: Rc::new(vec
                .into_iter()
                .map(|v| v.into())
                .collect())
        }
    }

    pub fn single<V: Into<Value>>(val: V) -> Self {
        Array::of(vec![val])
    }

    // pub fn push(&mut self, val: Value) {
    //     self.val.push(val)
    // }

    // pub fn pop(&mut self) -> Value {
    //     match self.val.pop() {
    //         Some(val) => val,
    //         None => Value::default(),
    //     }
    // }

    // pub fn get_mut(&mut self) -> &mut Vec<Value> {
    //     &mut self.val
    // }

    pub fn index(&self, nr: Nr) -> Value {
        let ix: usize = nr.floor().try_into().expect("value to large to use as index");
        match self.val.get(ix) {
            Some(val) => {
                val.fork()
            },
            None => Value::default(),
        }
    }

    pub fn iter(&self) -> Iter<'_, Value> {
        self.val.iter()
    }

    pub fn len(&self) -> usize {
        self.val.len()
    }
}

impl Fork for Array {
    fn fork(&self) -> Array {
        Array { val: self.val.clone() }
        //TODO @mark: use a better fork that can share part of the array
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut is_first = true;
        for item in &*self.val {
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
        for item in &*self.val {
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

// impl IntoIterator for Array {
//     type Item = Value;
//     type IntoIter = vec::IntoIter<Value>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.val.into_iter()
//     }
// }

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
