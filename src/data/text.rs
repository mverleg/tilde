use ::std::fmt;
use ::std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Text {
    val: Rc<String>,
}

impl Text {
    pub fn of(val: impl Into<String>) -> Self {
        Text { val: Rc::new(val.into()) }
    }

    pub fn empty() -> Self {
        Self::of("")
    }

    pub fn as_str(&self) -> &str {
        &self.val
    }

    pub fn len(&self) -> usize {
        self.val.len()
    }

    pub fn fork(&self) -> Text {
        Text { val: self.val.clone() }
        //TODO @mark: use a better fork that can share part of the string?
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.val)
    }
}

impl From<String> for Text {
    fn from(val: String) -> Self {
        Text::of(val)
    }
}
