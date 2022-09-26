use std::fmt;

use super::Float10;

#[derive(Debug, PartialEq)]
pub struct ScalingFactors {
    first: [char; 2],
    second: Float10,
}

impl fmt::Display for ScalingFactors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.first, self.second)
    }
}
