use std::fmt;

use super::CrossingBond;

/// A single-atom subgraph proxy. Its coordinate is not provided because
/// display requires re-assignment of global coordinates. Crossing bonds are
/// used to do this.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Superatom {
    /// Superatom text, which may include formatting. The label is centered
    /// at the common atom for all crossing bonds.
    pub label: String,
    /// Bonds crossing into the superatom.
    pub crossing_bonds: Vec<CrossingBond>,
}

impl fmt::Display for Superatom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.crossing_bonds.is_empty() {
            write!(f, "LABEL={}", self.label)
        } else {
            write!(
                f,
                "XBONDS=({}{}){} LABEL={}",
                self.crossing_bonds.len(),
                self.crossing_bonds
                    .iter()
                    .map(|b| format!(" {}", b.index))
                    .collect::<String>(),
                self.crossing_bonds
                    .iter()
                    .map(|b| format!(" {}", b))
                    .collect::<String>(),
                self.label
            )
        }
    }
}

#[cfg(test)]
mod to_string {
    use crate::ctab::{Coordinate, Index};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn empty() {
        let superatom = Superatom {
            label: "X".to_string(),
            crossing_bonds: vec![],
        };

        assert_eq!(superatom.to_string(), "LABEL=X")
    }

    #[test]
    fn full() {
        let superatom = Superatom {
            label: "X".to_string(),
            crossing_bonds: vec![
                CrossingBond {
                    index: Index::try_from("13").unwrap(),
                    coordinate: Coordinate::new(1.1, 2.2, 0.),
                },
                CrossingBond {
                    index: Index::try_from("42").unwrap(),
                    coordinate: Coordinate::new(3.3, 4.4, 0.),
                },
            ],
        };

        assert_eq!(
            superatom.to_string(),
            "XBONDS=(2 13 42) CSTATE=(4 13 1.1 2.2 0) CSTATE=(4 42 3.3 4.4 0) LABEL=X"
        )
    }
}
