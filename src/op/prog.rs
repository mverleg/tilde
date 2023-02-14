use ::std::fmt::Write;
use ::std::ops::Index;
pub use ::std::slice;
use ::std::vec;
use crate::compile::Letter;

use crate::op::Op;
use crate::op::typ::Typ;
use crate::TildeRes;

#[derive(Debug, PartialEq)]
pub struct Prog {
    ops: Vec<Op>,
}

impl Prog {
    pub fn of(ops: Vec<Op>) -> Self {
        Prog { ops }
    }

    pub fn get(&self, index: usize) -> Option<&Op> {
        self.ops.get(index)
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    // pub fn iter(&self) -> slice::Iter<Op> {
    //     self.ops.iter()
    // }
    //
    // pub fn into_iter(self) -> vec::IntoIter<Op> {
    //     self.ops.into_iter()
    // }
    //TODO @mverleg:
}

impl Index<usize> for Prog {
    type Output = Op;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ops[index]
    }
}

impl Prog {
    pub fn long_code(&self) -> String {
        let mut code = String::with_capacity(self.ops.len() * 16);
        for op in &self.ops {
            write!(code, "{} ", op.long_code().as_ref()).unwrap()  //TODO @mverleg:
        }
        code.pop();
        code
    }

    fn golf_letters(&self) -> TildeRes<Vec<Letter>> {
        //TODO @mark: cache?
        let mut letters = Vec::with_capacity(self.ops.len() * 4);
        for op in &self.ops {
            letters.extend(op.golf_code()?)
        }
        //TODO @mark: make a minify function with more tricks
        if letters.last() == Some(&Letter::Text) {
            letters.pop();
        }
        Ok(letters)
    }

    pub fn golf_len(&self) -> TildeRes<usize> {
        Ok((self.golf_letters()?.len() + 1) / 2)
    }

    pub fn golf_code(&self) -> TildeRes<String> {
        let mut code = String::with_capacity(self.ops.len() * 4);
        for letter in self.golf_letters()? {
            write!(code, "{}", letter.symbol()).unwrap()
        }
        Ok(code)
    }

    pub fn golf_code_b64(&self) -> TildeRes<String> {
        let mut bytes = Vec::with_capacity(self.ops.len() * 4);
        let letters = self.golf_letters()?;
        let mut i = 0;
        while i + 1 < letters.len() {
            bytes.push(16 * letters[i].nr() + letters[i + 1].nr());
            i += 2;
        }
        if i < letters.len() {
            bytes.push(16 * letters[i].nr())
            //TODO @mark: need to to something to make the last letter not interpreted (or no-op)
        }
        //TODO @mark: bas64
        Ok("todo-golf-text-b64".to_owned())
    }
}
