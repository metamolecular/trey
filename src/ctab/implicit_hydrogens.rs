use super::{Atom, AtomKind, Charge, Element};

pub fn implicit_hydrogens(atom: &Atom, bond_order_sum: usize) -> Option<usize> {
    if let Some(custom) = &atom.valence {
        let custom = usize::from(custom);

        if bond_order_sum <= custom {
            return Some(custom - bond_order_sum);
        } else {
            return Some(0);
        }
    }

    let element = match &atom.kind {
        AtomKind::Element(element) => match isoelectronic_element(element, &atom.charge) {
            Some(element) => element,
            None => return None,
        },
        _ => return None,
    };
    let targets = match default_valences(&element) {
        Some(targets) => targets,
        None => return None,
    };
    let target = match select_target(bond_order_sum, targets) {
        Some(target) => target,
        None => return Some(0),
    };

    Some(target as usize - bond_order_sum)
}

fn isoelectronic_element(element: &Element, charge: &Charge) -> Option<Element> {
    let effective_charge: i8 = charge.into();

    match effective_charge {
        -15 => match element {
            Element::H => Some(Element::P),
            Element::He => Some(Element::Ar),
            _ => None,
        },
        -14 => match element {
            Element::H => Some(Element::Si),
            Element::He => Some(Element::P),
            Element::B => Some(Element::Ar),
            _ => None,
        },
        -13 => match element {
            Element::B => Some(Element::Cl),
            Element::C => Some(Element::Ar),
            _ => None,
        },
        -12 => match element {
            Element::B => Some(Element::S),
            Element::C => Some(Element::Cl),
            Element::N => Some(Element::Ar),
            _ => None,
        },
        -11 => match element {
            Element::B => Some(Element::P),
            Element::C => Some(Element::S),
            Element::N => Some(Element::Cl),
            Element::O => Some(Element::Ar),
            _ => None,
        },
        -10 => match element {
            Element::B => Some(Element::Si),
            Element::C => Some(Element::P),
            Element::N => Some(Element::S),
            Element::O => Some(Element::Cl),
            Element::F => Some(Element::Ar),
            _ => None,
        },
        -9 => match element {
            Element::H => Some(Element::Ne),
            _ => None,
        },
        -8 => match element {
            Element::H => Some(Element::F),
            Element::He => Some(Element::Ne),
            Element::Ne => Some(Element::Cl),
            _ => None,
        },
        -7 => match element {
            Element::H => Some(Element::O),
            Element::He => Some(Element::F),
            Element::Ne => Some(Element::S),
            _ => None,
        },
        -6 => match element {
            Element::H => Some(Element::N),
            Element::He => Some(Element::O),
            Element::Ne => Some(Element::P),
            _ => None,
        },
        -5 => match element {
            Element::H => Some(Element::B),
            Element::He => Some(Element::N),
            Element::B => Some(Element::Ne),
            Element::Ne => Some(Element::Si),
            _ => None,
        },
        -4 => match element {
            Element::H => Some(Element::B),
            Element::He => Some(Element::C),
            Element::B => Some(Element::F),
            Element::C => Some(Element::Ne),
            Element::N | Element::O | Element::F | Element::Ne => None,
            Element::Si => Some(Element::Ar),
            _ => None,
        },
        -3 => match element {
            Element::H => Some(Element::C),
            Element::He => Some(Element::B),
            Element::B => Some(Element::O),
            Element::C => Some(Element::F),
            Element::N => Some(Element::Ne),
            Element::O | Element::F | Element::Ne => None,
            Element::Si => Some(Element::Cl),
            Element::P => Some(Element::Ar),
            Element::S | Element::Cl | Element::Ar => None,
            Element::As => Some(Element::Kr),
            Element::Se | &Element::Br | Element::Kr => None,
            _ => None,
        },
        -2 => match element {
            Element::H => Some(Element::B),
            Element::He => None,
            Element::B => Some(Element::N),
            Element::C => Some(Element::O),
            Element::N => Some(Element::F),
            Element::O => Some(Element::Ne),
            Element::F => None,
            Element::Ne => None,
            Element::Si => Some(Element::S),
            Element::P => Some(Element::Cl),
            Element::S => Some(Element::Ar),
            Element::Cl => None,
            Element::Ar => None,
            Element::As => Some(Element::Br),
            Element::Se => Some(Element::Kr),
            Element::Br => None,
            Element::Kr => None,
            Element::Te => Some(Element::Xe),
            _ => None,
        },
        -1 => match element {
            Element::H => Some(Element::He),
            Element::B => Some(Element::C),
            Element::C => Some(Element::N),
            Element::N => Some(Element::O),
            Element::O => Some(Element::F),
            Element::F => Some(Element::Ne),
            Element::Al => Some(Element::Si),
            Element::Si => Some(Element::P),
            Element::P => Some(Element::S),
            Element::S => Some(Element::Cl),
            Element::Cl => Some(Element::Ar),
            Element::As => Some(Element::Se),
            Element::Se => Some(Element::Br),
            Element::Br => Some(Element::Kr),
            Element::Te => Some(Element::I),
            Element::I => Some(Element::Xe),
            Element::At => Some(Element::Rn),
            _ => None,
        },
        0 => Some(element.clone()),
        1 => match element {
            Element::H => None,
            Element::B => None,
            Element::C => Some(Element::B),
            Element::N => Some(Element::C),
            Element::O => Some(Element::N),
            Element::F => Some(Element::O),
            Element::Ne => Some(Element::F),
            Element::P => Some(Element::Si),
            Element::S => Some(Element::P),
            Element::Cl => Some(Element::S),
            Element::Ar => Some(Element::Cl),
            Element::Se => Some(Element::As),
            Element::Br => Some(Element::Se),
            Element::Kr => Some(Element::Br),
            Element::I => Some(Element::Te),
            Element::Xe => Some(Element::I),
            Element::Rn => Some(Element::At),
            _ => None,
        },
        2 => match element {
            Element::N => Some(Element::B),
            Element::O => Some(Element::C),
            Element::F => Some(Element::N),
            Element::Ne => Some(Element::O),
            Element::S => Some(Element::Si),
            Element::Cl => Some(Element::P),
            Element::Ar => Some(Element::S),
            Element::Br => Some(Element::As),
            Element::Kr => Some(Element::Se),
            Element::Xe => Some(Element::Te),
            _ => None,
        },
        3 => match element {
            Element::B => Some(Element::He),
            Element::O => Some(Element::B),
            Element::F => Some(Element::C),
            Element::Ne => Some(Element::N),
            Element::Cl => Some(Element::Si),
            Element::Ar => Some(Element::P),
            Element::Kr => Some(Element::As),
            _ => None,
        },
        4 => match element {
            Element::F => Some(Element::B),
            Element::Ne => Some(Element::C),
            Element::Ar => Some(Element::Si),
            _ => None,
        },
        5 => match element {
            Element::C => Some(Element::H),
            Element::N => Some(Element::He),
            Element::Ne => Some(Element::B),
            Element::Si => Some(Element::F),
            Element::P => Some(Element::Ne),
            _ => None,
        },
        6 => match element {
            Element::N => Some(Element::H),
            Element::O => Some(Element::He),
            Element::Si => Some(Element::O),
            Element::P => Some(Element::F),
            Element::S => Some(Element::Ne),
            _ => None,
        },
        7 => match element {
            Element::O => Some(Element::H),
            Element::F => Some(Element::He),
            Element::Si => Some(Element::N),
            Element::P => Some(Element::O),
            Element::S => Some(Element::F),
            Element::Cl => Some(Element::Ne),
            _ => None,
        },
        8 => match element {
            Element::F => Some(Element::H),
            Element::Si => Some(Element::C),
            Element::P => Some(Element::N),
            Element::S => Some(Element::O),
            Element::Cl => Some(Element::F),
            Element::Ar => Some(Element::Ne),
            _ => None,
        },
        9 => match element {
            Element::Ne => Some(Element::H),
            Element::Si => Some(Element::B),
            Element::P => Some(Element::C),
            Element::S => Some(Element::N),
            Element::Cl => Some(Element::O),
            Element::Ar => Some(Element::F),
            _ => None,
        },
        10 => match element {
            Element::P => Some(Element::B),
            Element::S => Some(Element::C),
            Element::Cl => Some(Element::N),
            Element::Ar => Some(Element::O),
            _ => None,
        },
        11 => match element {
            Element::S => Some(Element::B),
            Element::Cl => Some(Element::C),
            Element::Ar => Some(Element::N),
            _ => None,
        },
        12 => match element {
            Element::Cl => Some(Element::B),
            Element::Ar => Some(Element::C),
            _ => None,
        },
        13 => match element {
            Element::Si => Some(Element::H),
            Element::P => Some(Element::He),
            Element::Ar => Some(Element::B),
            _ => None,
        },
        14 => match element {
            Element::P => Some(Element::H),
            Element::S => Some(Element::He),
            _ => None,
        },
        15 => match element {
            Element::S => Some(Element::H),
            _ => None,
        },
        _ => unreachable!(),
    }
}

fn default_valences(element: &Element) -> Option<&[u8]> {
    match element {
        Element::H => Some(&[1]),
        Element::He => Some(&[0]),
        Element::B => Some(&[3]),
        Element::C => Some(&[4]),
        Element::N => Some(&[3]),
        Element::O => Some(&[2]),
        Element::F => Some(&[1]),
        Element::Ne => Some(&[0]),
        Element::Si => Some(&[4]),
        Element::P => Some(&[3, 5]),
        Element::S => Some(&[2, 4, 6]),
        Element::Cl => Some(&[1, 3, 5, 7]),
        Element::Ar => Some(&[0]),
        Element::As => Some(&[3, 5]),
        Element::Se => Some(&[2, 4, 6]),
        Element::Br => Some(&[1]),
        Element::Kr => Some(&[0]),
        Element::Te => Some(&[2, 4, 6]),
        Element::I => Some(&[1, 3, 5, 7]),
        Element::Xe => Some(&[0]),
        Element::At => Some(&[1, 3, 5, 7]),
        Element::Rn => Some(&[0]),
        _ => None,
    }
}

fn select_target(query: usize, targets: &[u8]) -> Option<u8> {
    targets
        .iter()
        .find(|&target| *target as usize >= query)
        .map(|v| *v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctab::Valence;
    use pretty_assertions::assert_eq;

    #[test]
    fn any() {
        let atom = Atom {
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 0), None)
    }

    #[test]
    fn subvalent_any_with_custom() {
        let atom = Atom {
            valence: Some(Valence::try_from(1).unwrap()),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 0), Some(1))
    }

    #[test]
    fn homovalent_any_with_custom() {
        let atom = Atom {
            valence: Some(Valence::try_from(1).unwrap()),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 1), Some(0))
    }

    #[test]
    fn supervalent_any_with_custom() {
        let atom = Atom {
            valence: Some(Valence::try_from(1).unwrap()),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 2), Some(0))
    }

    #[test]
    fn polymer_bead() {
        let atom = Atom {
            kind: AtomKind::PolymerBead,
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 0), None)
    }

    #[test]
    fn element_no_defaults() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Sn),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 3), None)
    }

    #[test]
    fn subvalent_element_no_defaults_custom() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Sn),
            valence: Some(Valence::try_from(2).unwrap()),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 1), Some(1))
    }

    #[test]
    fn supervalent_element_no_defaults_custom() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Sn),
            valence: Some(Valence::try_from(2).unwrap()),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 3), Some(0))
    }

    #[test]
    fn subvalent_element_neutral() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 1), Some(3))
    }

    #[test]
    fn homovalent_element_neutral() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 4), Some(0))
    }

    #[test]
    fn supervalent_element_neutral() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 5), Some(0))
    }

    #[test]
    fn subvalent_element_charged() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            charge: Charge::try_from(1).unwrap(),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 2), Some(1))
    }

    #[test]
    fn homovalent_element_charged() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            charge: Charge::try_from(1).unwrap(),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 3), Some(0))
    }

    #[test]
    fn supervalent_element_charged() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            charge: Charge::try_from(1).unwrap(),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 4), Some(0))
    }

    #[test]
    fn subvalent_element_charged_off() {
        let atom = Atom {
            kind: AtomKind::Element(Element::C),
            charge: Charge::try_from(2).unwrap(),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 3), None)
    }

    #[test]
    fn subvalent_element_aluminum() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Al),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 3), None)
    }

    #[test]
    fn subvalent_element_aluminum_minus_one() {
        let atom = Atom {
            kind: AtomKind::Element(Element::Al),
            charge: Charge::try_from(-1).unwrap(),
            ..Default::default()
        };

        assert_eq!(implicit_hydrogens(&atom, 3), Some(1))
    }
}

#[cfg(test)]
mod select_target {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn target_less_than_first() {
        let query = 0;

        assert_eq!(select_target(query, &[1]), Some(1))
    }

    #[test]
    fn target_equal_to_first() {
        let query = 1;

        assert_eq!(select_target(query, &[1]), Some(1))
    }

    #[test]
    fn target_between_first_and_second() {
        let query = 2;

        assert_eq!(select_target(query, &[1, 3]), Some(3))
    }

    #[test]
    fn target_after_first() {
        let query = 4;

        assert_eq!(select_target(query, &[1, 3]), None)
    }
}
