use std::hash::{Hash, Hasher};
use std::{convert, fmt};

use super::Error;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Index(String);

impl Index {
    pub fn new(id: usize) -> Self {
        id.to_string().try_into().unwrap()
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Index {
    fn default() -> Self {
        Self("1".into())
    }
}

impl Hash for Index {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl convert::TryFrom<String> for Index {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::IdFormat);
        }

        for (i, character) in value.chars().enumerate() {
            match character {
                '0' => {
                    if i == 0 {
                        return Err(Error::IdFormat);
                    }
                }
                '1'..='9' => (),
                _ => return Err(Error::IdFormat),
            }
        }

        Ok(Self(value))
    }
}

impl convert::TryFrom<&str> for Index {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl convert::TryFrom<usize> for Index {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl convert::TryFrom<u32> for Index {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl From<&Index> for u32 {
    fn from(value: &Index) -> Self {
        value.0.parse().unwrap()
    }
}

#[cfg(test)]
mod try_from {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        assert_eq!(Index::try_from(""), Err(Error::IdFormat))
    }

    #[test]
    fn leading_zero_string() {
        assert_eq!(Index::try_from("01"), Err(Error::IdFormat))
    }

    #[test]
    fn non_numeric_string() {
        assert_eq!(Index::try_from("foobar"), Err(Error::IdFormat))
    }

    #[test]
    fn valid_string() {
        assert_eq!(Index::try_from("10").unwrap().to_string(), "10")
    }
}

#[cfg(test)]
mod equals {
    use super::*;

    #[test]
    fn same() {
        let first = Index::try_from("1").unwrap();
        let second = Index::try_from("1").unwrap();

        assert!(first == second)
    }

    #[test]
    fn different() {
        let first = Index::try_from("1").unwrap();
        let second = Index::try_from("2").unwrap();

        assert!(first != second)
    }
}

#[cfg(test)]
mod hash_map {
    use std::{collections::HashMap, convert::TryFrom};

    use super::*;

    #[test]
    fn same() {
        let mut ids = HashMap::new();

        ids.insert(Index::try_from("1").unwrap(), 1);

        assert!(ids.contains_key(&Index::try_from("1").unwrap()))
    }

    #[test]
    fn different() {
        let mut ids = HashMap::new();

        ids.insert(Index::try_from("1").unwrap(), 1);

        assert!(!ids.contains_key(&Index::try_from("2").unwrap()))
    }
}
