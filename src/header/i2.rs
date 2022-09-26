use super::{fortran_int, Error};

#[derive(Debug, PartialEq)]
pub struct I2(u8);

impl I2 {
    pub fn new(digits: [char; 2]) -> Result<Self, Error> {
        Ok(Self(fortran_int(digits.iter())? as u8))
    }
}
