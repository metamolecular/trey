use std::{convert::TryFrom, fmt};

use super::Error;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Charge(i8);

impl Charge {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl TryFrom<i8> for Charge {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value > -16 && value < 16 {
            Ok(Charge(value))
        } else {
            Err(Error::InvalidCharge)
        }
    }
}

impl From<&Charge> for i8 {
    fn from(value: &Charge) -> Self {
        value.0.clone()
    }
}

impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod try_from {
    use super::*;

    #[test]
    fn inside_range() {
        assert_eq!(Charge::try_from(2), Ok(Charge(2)))
    }

    #[test]
    fn outside_range() {
        assert_eq!(Charge::try_from(16), Err(Error::InvalidCharge))
    }
}
