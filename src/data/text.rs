use ::std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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
}

impl fmt::Display for Text {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl From<String> for Text {
    fn from(val: String) -> Self {
        Text::of(val)
    }
}