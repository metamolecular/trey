use std::fmt;

use super::{Coordinate, Error, Index};

/// A bond crossing a substructure boundary.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct CrossingBond {
    /// Index of the bond.
    pub index: Index,
    /// Vector centered at the interior atom.
    pub coordinate: Coordinate,
}

impl CrossingBond {
    pub fn new(index: usize, x: f32, y: f32) -> Result<Self, Error> {
        Ok(Self {
            index: Index::try_from(index)?,
            coordinate: Coordinate::new(x, y, 0.),
        })
    }
}

impl fmt::Display for CrossingBond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CSTATE=(4 {} {})", self.index, self.coordinate)
    }
}

#[cfg(test)]
mod to_string {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        let crossing_bond = CrossingBond {
            index: Index::try_from("13").unwrap(),
            coordinate: Coordinate::new(1.1, 2.2, 0.),
        };

        assert_eq!(crossing_bond.to_string(), "CSTATE=(4 13 1.1 2.2 0)")
    }
}
