use std::fmt;

use super::{CoordinationDisplay, HydrogenDisplay};

#[derive(PartialEq, Debug, Clone)]
pub enum BondKind {
    Single,
    Double,
    Triple,
    Aromatic,
    SingleOrDouble,
    SingleOrAromatic,
    DoubleOrAromatic,
    Any,
    Coordination(Option<CoordinationDisplay>),
    Hydrogen(HydrogenDisplay),
}

impl Default for BondKind {
    fn default() -> Self {
        Self::Single
    }
}

impl fmt::Display for BondKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BondKind::Single => "1",
                BondKind::Double => "2",
                BondKind::Triple => "3",
                BondKind::Aromatic => "4",
                BondKind::SingleOrDouble => "5",
                BondKind::SingleOrAromatic => "6",
                BondKind::DoubleOrAromatic => "7",
                BondKind::Any => "8",
                BondKind::Coordination(_) => "9",
                BondKind::Hydrogen(_) => "10",
            }
        )
    }
}
