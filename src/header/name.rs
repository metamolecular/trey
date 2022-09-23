use std::{convert, fmt};

use super::Error;

#[derive(Debug, PartialEq, Default)]
pub struct Name(String);

impl convert::TryFrom<&str> for Name {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 80 {
            return Err(Error::StringTooLong);
        } else if value.contains("$RXN") || value.contains("$MDL") || value.contains("$$$$") {
            return Err(Error::ReservedTag);
        } else {
            Ok(Self(value.to_owned()))
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn too_long() {
        let string = (0..82).map(|_| "X").collect::<String>();

        assert_eq!(Name::try_from(string.as_str()), Err(Error::StringTooLong))
    }

    #[test]
    fn rxn() {
        assert_eq!(Name::try_from("$RXN"), Err(Error::ReservedTag))
    }

    #[test]
    fn mdl() {
        assert_eq!(Name::try_from("$MDL"), Err(Error::ReservedTag))
    }

    #[test]
    fn sdf_separator() {
        assert_eq!(Name::try_from("$$$$"), Err(Error::ReservedTag))
    }

    #[test]
    fn valid() {
        assert_eq!(Name::try_from("foo").unwrap().to_string(), "foo")
    }
}
