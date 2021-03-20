use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Description(String);

impl AsRef<str> for Description {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<S> From<S> for Description
where
    String: From<S>,
{
    fn from(from: S) -> Self {
        Self(String::from(from))
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
