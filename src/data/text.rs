use ::std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Text {
    val: String,
}

impl Text {
    pub fn of(val: impl Into<String>) -> Self {
        Text { val: val.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.val
    }

    pub fn len(&self) -> usize {
        self.val.len()
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
