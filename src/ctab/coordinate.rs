use std::fmt;

use super::Decimal;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Coordinate {
    pub x: Decimal,
    pub y: Decimal,
    pub z: Decimal,
}

impl Coordinate {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
