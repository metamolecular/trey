use super::{fortran_int, Error};

#[derive(Debug, PartialEq)]
pub struct I6(u32);

impl I6 {
    pub fn new(digits: [char; 6]) -> Result<Self, Error> {
        Ok(Self(fortran_int(digits.iter())? as u32))
    }
}
