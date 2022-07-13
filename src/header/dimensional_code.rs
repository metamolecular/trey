use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DimensionalCode {
    D2,
    D3,
}

impl Default for DimensionalCode {
    fn default() -> Self {
        Self::D2
    }
}

impl fmt::Display for DimensionalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::D2 => "2D",
                Self::D3 => "3D",
            }
        )
    }
}
