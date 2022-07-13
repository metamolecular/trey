use core::fmt;
use std::convert;

use super::Error;

#[derive(Debug, PartialEq, Default)]
pub struct Comment(String);

impl convert::TryFrom<&str> for Comment {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 80 {
            return Err(Error::StringTooLong);
        }

        Ok(Self(value.to_owned()))
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod try_from {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn too_long() {
        let string = (0..82).map(|_| "X").collect::<String>();

        assert_eq!(
            Comment::try_from(string.as_str()),
            Err(Error::StringTooLong)
        )
    }

    #[test]
    fn valid() {
        assert_eq!(Comment::try_from("foo").unwrap().to_string(), "foo")
    }
}
