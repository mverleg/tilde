use ::std::fmt;

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
