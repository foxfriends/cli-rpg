use std::fmt::{self, Display, Formatter};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Name(String);

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<S> From<S> for Name
where
    String: From<S>,
{
    fn from(from: S) -> Self {
        Self(String::from(from))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
