use std::fmt::Display;

use super::{fortran_float, Error};

#[derive(Debug, PartialEq, Default)]
pub struct Float10(f32);

impl Float10 {
    pub fn new(chars: [char; 10]) -> Result<Self, Error> {
        Ok(Self(fortran_float(chars.iter(), 5)?))
    }
}

impl Display for Float10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>10.5}", self.0)
    }
}

impl From<f32> for Float10 {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod fmt {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        let float = Float10::from(0.);

        assert_eq!(float.to_string(), "   0.00000")
    }

    #[test]
    fn full() {
        let float = Float10::from(1234.56787);

        assert_eq!(float.to_string(), "1234.56787")
    }

    #[test]
    fn negative() {
        let float = Float10::from(-3.14);

        assert_eq!(float.to_string(), "  -3.14000")
    }
}
