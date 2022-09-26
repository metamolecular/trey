use std::fmt::Display;

use super::{fortran_float, Error};

#[derive(Debug, PartialEq)]
pub struct Float12(f32);

impl Float12 {
    pub fn new(chars: [char; 12]) -> Result<Self, Error> {
        Ok(Self(fortran_float(chars.iter(), 5)?))
    }
}

impl Display for Float12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>10.5}", self.0)
    }
}
