use std::fmt;

use super::{CrossingBond, Index, SubstructureKind, Superatom};

/// An induced subgraph over a connection table.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct Substructure {
    pub index: Index,
    /// Indexes of the atoms lying within the substructure.
    pub atoms: Vec<Index>,
    pub kind: SubstructureKind,
}

impl Substructure {
    pub fn superatom(
        index: Index,
        atoms: Vec<Index>,
        label: String,
        crossing_bonds: Vec<CrossingBond>,
    ) -> Self {
        Self {
            index,
            atoms,
            kind: SubstructureKind::Superatom(Superatom {
                label,
                crossing_bonds,
            }),
        }
    }
}

impl fmt::Display for Substructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} 0 ATOMS=({}) {}",
            self.index,
            match self.kind {
                SubstructureKind::Superatom(_) => "SUP",
            },
            if self.atoms.is_empty() {
                "0".to_string()
            } else {
                format!(
                    "{}{}",
                    self.atoms.len(),
                    self.atoms
                        .iter()
                        .map(|a| format!(" {}", a))
                        .collect::<String>()
                )
            },
            self.kind
        )
    }
}

#[cfg(test)]
mod to_string {
    use crate::ctab::{Coordinate, CrossingBond, Superatom};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn superatom() {
        let sub = Substructure {
            index: Index::try_from("7").unwrap(),
            atoms: vec![
                Index::try_from("13").unwrap(),
                Index::try_from("42").unwrap(),
            ],
            kind: SubstructureKind::Superatom(Superatom {
                label: "X".to_string(),
                crossing_bonds: vec![
                    CrossingBond {
                        index: Index::try_from("81").unwrap(),
                        coordinate: Coordinate::new(1.1, 2.2, 0.),
                    },
                    CrossingBond {
                        index: Index::try_from("66").unwrap(),
                        coordinate: Coordinate::new(4.4, 5.5, 0.),
                    },
                ],
            }),
        };

        assert_eq!(sub.to_string(), "7 SUP 0 ATOMS=(2 13 42) XBONDS=(2 81 66) CSTATE=(4 81 1.1 2.2 0) CSTATE=(4 66 4.4 5.5 0) LABEL=X")
    }
}
