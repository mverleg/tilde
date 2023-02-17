use ::std::cmp::Ordering;
use ::std::hash;
use ::std::hash::Hasher;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::num::ParseFloatError;
use ::std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Nr {
    value: f64,
}

impl Nr {
    pub fn new(value: f64) -> Self {
        if value.is_finite() {
            Nr { value }
        } else {
            Nr { value: 0.0 }
        }
    }
}

impl PartialEq for Nr {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl Eq for Nr {}

impl hash::Hash for Nr {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(&self.value.to_ne_bytes())
    }
}

impl PartialOrd for Nr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Nr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value)
            .expect("finite float does not have total ordering, this should never happen")
    }
}

impl From<f64> for Nr {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<f32> for Nr {
    fn from(value: f32) -> Self {
        Self::new(value.into())
    }
}

impl From<u32> for Nr {
    fn from(value: u32) -> Self {
        Self::new(value.into())
    }
}

impl From<u64> for Nr {
    fn from(value: u64) -> Self {
        Self::new(value as f64)  // there is no TryInto
    }
}

impl From<i32> for Nr {
    fn from(value: i32) -> Self {
        Self::new(value.into())
    }
}

impl From<i64> for Nr {
    fn from(value: i64) -> Self {
        Self::new(value as f64)  // there is no TryInto
    }
}

impl fmt::Display for Nr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for Nr {
    type Err = ParseFloatError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Nr::new(f64::from_str(text)?))
    }
}
