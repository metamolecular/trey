use std::fmt;

use super::{
    AtomKind, AttachmentPoint, Charge, Coordinate, Error, Index, Valence,
};

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Atom {
    pub index: Index,
    pub kind: AtomKind,
    pub charge: Charge,
    pub coordinate: Coordinate,
    pub atom_atom_mapping: Option<Index>,
    pub valence: Option<Valence>,
    pub mass: Option<usize>,
    pub attachment_point: Option<AttachmentPoint>,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}{}{}{}{}{}",
            self.index,
            self.kind,
            self.coordinate,
            match &self.atom_atom_mapping {
                Some(mapping) => mapping.to_string(),
                None => "0".to_string(),
            },
            if self.charge.is_zero() {
                "".to_string()
            } else {
                format!(" CHG={}", self.charge)
            },
            match &self.mass {
                Some(mass) => format!(" MASS={}", mass),
                None => "".to_string(),
            },
            match &self.valence {
                Some(valence) => format!(" VAL={}", valence),
                None => "".to_string(),
            },
            match &self.attachment_point {
                Some(index) => format!(" ATTCHPT={}", index.to_string()),
                None => "".to_string(),
            },
            match &self.kind {
                AtomKind::Rgroup(rgroups) => format!(" RGROUPS={}", rgroups),
                _ => "".to_string(),
            }
        )
    }
}

impl Atom {
    pub fn any(index: usize, x: f32, y: f32) -> Result<Self, Error> {
        Ok(Self {
            index: index.try_into()?,
            coordinate: Coordinate::new(x, y, 0.),
            ..Default::default()
        })
    }

    pub fn bead(index: usize, x: f32, y: f32) -> Result<Self, Error> {
        Ok(Self {
            index: index.try_into()?,
            coordinate: Coordinate::new(x, y, 0.),
            kind: AtomKind::PolymerBead,
            ..Default::default()
        })
    }

    pub fn implicit_hydrogens(&self, bond_order_sum: usize) -> Option<usize> {
        if let Some(valence) = &self.valence {
            let custom = u8::from(valence) as usize;

            if bond_order_sum <= custom {
                return Some(custom - bond_order_sum);
            } else {
                return Some(0);
            }
        }

        let element = match &self.kind {
            AtomKind::Element(element) => {
                match element.isoelectronic(&self.charge) {
                    Some(element) => element,
                    None => return None,
                }
            }
            _ => return None,
        };

        match element.default_valences() {
            Some(valences) => valences
                .iter()
                .find(|&target| *target as usize >= bond_order_sum)
                .map(|v| *v as usize - bond_order_sum)
                .or(Some(0)),
            None => None,
        }
    }

    pub fn set_valence(
        &mut self,
        virtual_hydrogens: usize,
        bond_order_sum: usize,
    ) -> Result<(), Error> {
        if let AtomKind::Element(element) = &self.kind {
            if let Some(iso) = element.isoelectronic(&self.charge) {
                if let Some(default_valences) = iso.default_valences() {
                    let valence = virtual_hydrogens + bond_order_sum;

                    for default_valence in default_valences {
                        if *default_valence as usize == valence {
                            return Ok(());
                        }
                    }
                } else if virtual_hydrogens == 0 {
                    return Ok(());
                }
            } else {
                return Ok(());
            }
        } else if virtual_hydrogens == 0 {
            return Ok(());
        }

        let new_valence =
            Valence::try_from(virtual_hydrogens + bond_order_sum)?;

        self.valence.replace(new_valence);

        Ok(())
    }
}

#[cfg(test)]
mod to_string {
    use crate::ctab::{Element, ElementList};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        let atom = Atom::default();

        assert_eq!(atom.to_string(), "1 * 0 0 0 0")
    }

    #[test]
    fn index() {
        let atom = Atom {
            index: Index::try_from("42").unwrap(),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "42 * 0 0 0 0")
    }

    #[test]
    fn element() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 C 0 0 0 0")
    }

    #[test]
    fn element_list() {
        let atom = Atom {
            kind: AtomKind::ElementList(ElementList {
                not: true,
                elements: vec![Element::C, Element::N, Element::O],
            }),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 NOT[C,N,O] 0 0 0 0")
    }

    #[test]
    fn coordinates() {
        let atom = Atom {
            coordinate: Coordinate::new(1.1, 2.2, 3.3),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 * 1.1 2.2 3.3 0")
    }

    #[test]
    fn atom_atom_mapping() {
        let atom = Atom {
            atom_atom_mapping: Some(Index::try_from("42").unwrap()),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 * 0 0 0 42")
    }

    #[test]
    fn charge() {
        let atom = Atom {
            charge: Charge::try_from(-1).unwrap(),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 * 0 0 0 0 CHG=-1")
    }

    #[test]
    fn charge_equals_zero() {
        let atom = Atom {
            charge: Charge::try_from(0).unwrap(),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 * 0 0 0 0")
    }

    #[test]
    fn valence_equals_zero() {
        let atom = Atom {
            valence: Some(Valence::try_from(0).unwrap()),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 * 0 0 0 0 VAL=-1")
    }

    #[test]
    fn valence_exceeds_zero() {
        let atom = Atom {
            valence: Some(Valence::try_from(3).unwrap()),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 * 0 0 0 0 VAL=3")
    }

    #[test]
    fn mass() {
        let atom = Atom {
            mass: Some(42),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 * 0 0 0 0 MASS=42")
    }

    #[test]
    fn attachment_point() {
        let atom = Atom {
            attachment_point: Some(AttachmentPoint::First),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 * 0 0 0 0 ATTCHPT=1")
    }

    #[test]
    fn rgroups() {
        let atom = Atom {
            kind: AtomKind::Rgroup(
                vec!["13".try_into().unwrap(), "42".try_into().unwrap()].into(),
            ),
            ..Default::default()
        };

        assert_eq!(atom.to_string(), "1 R# 0 0 0 0 RGROUPS=(2 13 42)")
    }

    #[test]
    fn kitchen_sink() {
        let atom = Atom {
            index: Index::try_from("42").unwrap(),
            kind: AtomKind::Element(Element::C),
            charge: Charge::try_from(1).unwrap(),
            coordinate: Coordinate::new(1.1, 2.2, 3.3),
            atom_atom_mapping: Some(Index::default()),
            valence: Some(Valence::try_from(3).unwrap()),
            mass: Some(12),
            attachment_point: Some(AttachmentPoint::First),
            ..Default::default()
        };

        assert_eq!(
            atom.to_string(),
            "42 C 1.1 2.2 3.3 1 CHG=1 MASS=12 VAL=3 ATTCHPT=1"
        )
    }
}

#[cfg(test)]
mod implicit_hydrogens {
    use crate::ctab::Element;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn any() {
        let atom = Atom {
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(0), None)
    }

    #[test]
    fn subvalent_any_with_custom() {
        let atom = Atom {
            valence: Some(Valence::try_from(1).unwrap()),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(0), Some(1))
    }

    #[test]
    fn homovalent_any_with_custom() {
        let atom = Atom {
            valence: Some(Valence::try_from(1).unwrap()),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(1), Some(0))
    }

    #[test]
    fn supervalent_any_with_custom() {
        let atom = Atom {
            valence: Some(Valence::try_from(1).unwrap()),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(2), Some(0))
    }

    #[test]
    fn polymer_bead() {
        let atom = Atom {
            kind: AtomKind::PolymerBead,
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(0), None)
    }

    #[test]
    fn element_no_defaults() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Sn),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(3), None)
    }

    #[test]
    fn subvalent_element_no_defaults_custom() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Sn),
            valence: Some(Valence::try_from(2).unwrap()),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(1), Some(1))
    }

    #[test]
    fn supervalent_element_no_defaults_custom() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Sn),
            valence: Some(Valence::try_from(2).unwrap()),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(3), Some(0))
    }

    #[test]
    fn subvalent_element_neutral() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(1), Some(3))
    }

    #[test]
    fn homovalent_element_neutral() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(4), Some(0))
    }

    #[test]
    fn supervalent_element_neutral() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(5), Some(0))
    }

    #[test]
    fn subvalent_element_charged() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            charge: Charge::try_from(1).unwrap(),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(2), Some(1))
    }

    #[test]
    fn homovalent_element_charged() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            charge: Charge::try_from(1).unwrap(),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(3), Some(0))
    }

    #[test]
    fn supervalent_element_charged() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            charge: Charge::try_from(1).unwrap(),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(4), Some(0))
    }

    #[test]
    fn subvalent_element_charged_off() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            charge: Charge::try_from(2).unwrap(),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(3), None)
    }

    #[test]
    fn subvalent_element_aluminum() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Al),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(3), None)
    }

    #[test]
    fn subvalent_element_aluminum_minus_one() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Al),
            charge: Charge::try_from(-1).unwrap(),
            ..Default::default()
        };

        assert_eq!(atom.implicit_hydrogens(3), Some(1))
    }
}

#[cfg(test)]
mod set_valence {
    use crate::ctab::Element;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn nonelement() {
        let mut atom = Atom {
            ..Default::default()
        };

        atom.set_valence(2, 2).unwrap();

        assert_eq!(atom.valence, Some(Valence::try_from(4).unwrap()))
    }

    #[test]
    fn element_with_charge() {
        let mut atom = Atom {
            kind: AtomKind::Element(Element::N),
            charge: Charge::try_from(-1).unwrap(),
            ..Default::default()
        };

        atom.set_valence(1, 1).unwrap();

        assert_eq!(atom.valence, None)
    }

    #[test]
    fn element_with_charge_without_defaults() {
        let mut atom = Atom {
            kind: AtomKind::Element(Element::Na),
            charge: Charge::try_from(1).unwrap(),
            ..Default::default()
        };

        atom.set_valence(0, 0).unwrap();

        assert_eq!(atom.valence, None)
    }

    #[test]
    fn nonelement_virtual_hydrogens_zero() {
        let mut atom = Atom {
            ..Default::default()
        };

        atom.set_valence(0, 2).unwrap();

        assert_eq!(atom.valence, None)
    }

    #[test]
    fn element_without_defaults() {
        let mut atom = Atom {
            kind: AtomKind::Element(Element::Fe),
            ..Default::default()
        };

        atom.set_valence(1, 2).unwrap();

        assert_eq!(atom.valence, Some(Valence::try_from(3).unwrap()))
    }

    #[test]
    fn element_metal_vh_zero_bos_one() {
        let mut atom = Atom {
            kind: AtomKind::Element(Element::V),
            ..Default::default()
        };

        atom.set_valence(0, 1).unwrap();

        assert_eq!(atom.valence, None)
    }

    #[test]
    fn element_with_matching_default() {
        let mut atom = Atom {
            kind: AtomKind::Element(Element::C),
            ..Default::default()
        };

        atom.set_valence(2, 2).unwrap();

        assert_eq!(atom.valence, None)
    }

    #[test]
    fn element_without_matching_default() {
        let mut atom = Atom {
            kind: AtomKind::Element(Element::C),
            ..Default::default()
        };

        atom.set_valence(2, 1).unwrap();

        assert_eq!(atom.valence, Some(Valence::try_from(3).unwrap()))
    }

    #[test]
    fn element_with_default_zero_valent() {
        let mut atom = Atom {
            kind: AtomKind::Element(Element::O),
            ..Default::default()
        };

        atom.set_valence(0, 0).unwrap();

        assert_eq!(atom.valence, Some(Valence::try_from(0).unwrap()))
    }
}
