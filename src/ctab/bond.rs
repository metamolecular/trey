use std::fmt;

use super::{BondConfiguration, BondKind, Error, Index};

#[derive(PartialEq, Default, Debug, Clone)]
pub struct Bond {
    pub index: Index,
    pub configuration: Option<BondConfiguration>,
    pub atom1: Index,
    pub atom2: Index,
    pub kind: BondKind,
}

impl Bond {
    pub fn single(id: usize, atom1: usize, atom2: usize) -> Result<Self, Error> {
        Ok(Self {
            index: id.try_into()?,
            atom1: atom1.try_into()?,
            atom2: atom2.try_into()?,
            ..Default::default()
        })
    }

    pub fn is_stereo(&self) -> bool {
        match self.kind {
            BondKind::Single => match self.configuration {
                Some(BondConfiguration::Up | BondConfiguration::Down) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn valence_contribution(&self) -> u8 {
        match &self.kind {
            BondKind::Single => 1,
            BondKind::Double => 2,
            BondKind::Triple => 3,
            _ => 0,
        }
    }

    pub fn contains(&self, index: &Index) -> bool {
        &self.atom1 == index || &self.atom2 == index
    }

    pub fn mate(&self, index: &Index) -> Option<Index> {
        if &self.atom1 == index {
            Some(self.atom2.clone())
        } else if &self.atom2 == index {
            Some(self.atom1.clone())
        } else {
            None
        }
    }
}

impl fmt::Display for Bond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}{}{}",
            self.index,
            self.kind,
            self.atom1,
            self.atom2,
            match &self.configuration {
                Some(configuration) => format!(" CFG={}", configuration),
                None => "".to_string(),
            },
            match &self.kind {
                BondKind::Coordination(display) => match display {
                    Some(display) => format!(" DISP={}", display),
                    None => "".to_string(),
                },
                BondKind::Hydrogen(display) => format!(" DISP={}", display),
                _ => "".to_string(),
            }
        )
    }
}

#[cfg(test)]
mod is_stereo {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn double_up() {
        let bond = Bond {
            kind: BondKind::Double,
            configuration: Some(BondConfiguration::Up),
            ..Default::default()
        };

        assert_eq!(bond.is_stereo(), false)
    }
}

#[cfg(test)]
mod to_string {
    use crate::ctab::{CoordinationDisplay, HydrogenDisplay};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        let bond = Bond {
            ..Default::default()
        };

        assert_eq!(bond.to_string(), "1 1 1 1")
    }

    #[test]
    fn index() {
        let bond = Bond {
            index: Index::try_from("42").unwrap(),
            ..Default::default()
        };

        assert_eq!(bond.to_string(), "42 1 1 1")
    }

    #[test]
    fn coordination() {
        let bond = Bond {
            kind: BondKind::Coordination(Some(CoordinationDisplay::Dative)),
            ..Default::default()
        };

        assert_eq!(bond.to_string(), "1 9 1 1 DISP=DATIVE")
    }

    #[test]
    fn hydrogen() {
        let bond = Bond {
            kind: BondKind::Hydrogen(HydrogenDisplay::HBond1),
            ..Default::default()
        };

        assert_eq!(bond.to_string(), "1 10 1 1 DISP=HBOND1")
    }

    #[test]
    fn kitchen_sink() {
        let bond = Bond {
            atom1: Index::try_from("13").unwrap(),
            atom2: Index::try_from("42").unwrap(),
            kind: BondKind::Single,
            configuration: Some(BondConfiguration::Up),
            ..Default::default()
        };

        assert_eq!(bond.to_string(), "1 1 13 42 CFG=1")
    }
}
