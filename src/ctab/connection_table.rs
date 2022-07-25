use std::collections::HashMap;

use super::{Atom, Bond, Collection, Error, Index, Substructure, SubstructureKind};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ConnectionTable {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
    pub collections: Vec<Collection>,
    pub substructures: Vec<Substructure>,
}

impl ConnectionTable {
    pub fn is_chiral(&self) -> bool {
        self.bonds.iter().find(|bond| bond.is_stereo()).is_some()
    }

    pub fn merge(&mut self, mut other: Self) {
        self.atoms.append(&mut other.atoms);
        self.bonds.append(&mut other.bonds);

        for mut other_collection in other.collections {
            let mut merged = false;

            for self_collection in self.collections.iter_mut() {
                if self_collection.merge(&mut other_collection) {
                    merged = true;

                    break;
                }
            }

            if !merged {
                self.collections.push(other_collection);
            }
        }

        self.substructures.append(&mut other.substructures);
    }

    pub fn reindex(&mut self) -> Result<(), Error> {
        let mut atom_indexes: HashMap<Index, Index> = HashMap::new();

        for (i, atom) in self.atoms.iter_mut().enumerate() {
            let mut index = (i + 1).try_into().expect("atom index");

            std::mem::swap(&mut atom.index, &mut index);

            if atom_indexes.insert(index, atom.index.clone()).is_some() {
                return Err(Error::DuplicateAtom);
            }
        }

        let mut bond_indexes: HashMap<Index, Index> = HashMap::new();

        for (i, bond) in self.bonds.iter_mut().enumerate() {
            let mut index = (i + 1).try_into().expect("bond index");

            std::mem::swap(&mut bond.index, &mut index);

            if bond_indexes.insert(index, bond.index.clone()).is_some() {
                return Err(Error::DuplicateBond);
            }

            bond.atom1 = atom_indexes
                .get(&bond.atom1)
                .ok_or(Error::MissingAtom)?
                .clone();
            bond.atom2 = atom_indexes
                .get(&bond.atom2)
                .ok_or(Error::MissingAtom)?
                .clone();
        }

        for collection in self.collections.iter_mut() {
            let indexes = match collection {
                Collection::AbsoluteStereo(indexes) => indexes,
                Collection::RelativeStereo(_, indexes) => indexes,
                Collection::RacemicStereo(_, indexes) => indexes,
            };

            for index in indexes.iter_mut() {
                let replacement = atom_indexes.get_mut(&index).ok_or(Error::MissingAtom)?;

                std::mem::swap(index, replacement);
            }
        }

        for substructure in self.substructures.iter_mut() {
            for atom in substructure.atoms.iter_mut() {
                let mut index = atom_indexes.get(&atom).ok_or(Error::MissingAtom)?.clone();
                std::mem::swap(atom, &mut index);
            }

            let superatom = match &mut substructure.kind {
                SubstructureKind::Superatom(superatom) => superatom,
            };

            for crossing_bond in superatom.crossing_bonds.iter_mut() {
                let mut index = bond_indexes
                    .get(&crossing_bond.index)
                    .ok_or(Error::MissingBond)?
                    .clone();

                std::mem::swap(&mut crossing_bond.index, &mut index)
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod is_chiral {
    use crate::ctab::BondConfiguration;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn not_chiral() {
        let ctab = ConnectionTable::default();

        assert_eq!(ctab.is_chiral(), false)
    }

    #[test]
    fn chiral() {
        let ctab = ConnectionTable {
            bonds: vec![Bond {
                configuration: Some(BondConfiguration::Up),
                ..Default::default()
            }],
            ..ConnectionTable::default()
        };

        assert_eq!(ctab.is_chiral(), true)
    }
}

#[cfg(test)]
mod merge {
    use super::*;
    use crate::ctab;
    use pretty_assertions::assert_eq;

    #[test]
    fn atoms() {
        let mut target = ConnectionTable {
            atoms: vec![ctab::Atom {
                index: "1".try_into().unwrap(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let source = ConnectionTable {
            atoms: vec![ctab::Atom {
                index: "2".try_into().unwrap(),
                ..Default::default()
            }],
            ..Default::default()
        };

        target.merge(source);

        assert_eq!(
            target,
            ConnectionTable {
                atoms: vec![
                    ctab::Atom {
                        index: "1".try_into().unwrap(),
                        ..Default::default()
                    },
                    ctab::Atom {
                        index: "2".try_into().unwrap(),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        )
    }

    #[test]
    fn bonds() {
        let mut target = ConnectionTable {
            bonds: vec![ctab::Bond {
                index: "3".try_into().unwrap(),
                atom1: "1".try_into().unwrap(),
                atom2: "2".try_into().unwrap(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let source = ConnectionTable {
            bonds: vec![ctab::Bond {
                index: "4".try_into().unwrap(),
                atom1: "1".try_into().unwrap(),
                atom2: "3".try_into().unwrap(),
                ..Default::default()
            }],
            ..Default::default()
        };

        target.merge(source);

        assert_eq!(
            target,
            ConnectionTable {
                bonds: vec![
                    ctab::Bond {
                        index: "3".try_into().unwrap(),
                        atom1: "1".try_into().unwrap(),
                        atom2: "2".try_into().unwrap(),
                        ..Default::default()
                    },
                    ctab::Bond {
                        index: "4".try_into().unwrap(),
                        atom1: "1".try_into().unwrap(),
                        atom2: "3".try_into().unwrap(),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        )
    }

    #[test]
    fn substructures() {
        let mut target = ConnectionTable {
            substructures: vec![ctab::Substructure {
                index: "1".try_into().unwrap(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let source = ConnectionTable {
            substructures: vec![ctab::Substructure {
                index: "2".try_into().unwrap(),
                ..Default::default()
            }],
            ..Default::default()
        };

        target.merge(source);

        assert_eq!(
            target,
            ConnectionTable {
                substructures: vec![
                    ctab::Substructure {
                        index: "1".try_into().unwrap(),
                        ..Default::default()
                    },
                    ctab::Substructure {
                        index: "2".try_into().unwrap(),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        )
    }

    #[test]
    fn collections_empty_absolute() {
        let mut target = ConnectionTable {
            collections: vec![],
            ..Default::default()
        };
        let source = ConnectionTable {
            collections: vec![ctab::Collection::AbsoluteStereo(vec!["2"
                .try_into()
                .unwrap()])],
            ..Default::default()
        };

        target.merge(source);

        assert_eq!(
            target,
            ConnectionTable {
                collections: vec![ctab::Collection::AbsoluteStereo(vec!["2"
                    .try_into()
                    .unwrap()])],
                ..Default::default()
            }
        )
    }

    #[test]
    fn collections_absolute_absolute() {
        let mut target = ConnectionTable {
            collections: vec![ctab::Collection::AbsoluteStereo(vec!["1"
                .try_into()
                .unwrap()])],
            ..Default::default()
        };
        let source = ConnectionTable {
            collections: vec![ctab::Collection::AbsoluteStereo(vec!["2"
                .try_into()
                .unwrap()])],
            ..Default::default()
        };

        target.merge(source);

        assert_eq!(
            target,
            ConnectionTable {
                collections: vec![ctab::Collection::AbsoluteStereo(vec![
                    "1".try_into().unwrap(),
                    "2".try_into().unwrap()
                ])],
                ..Default::default()
            }
        )
    }
}

#[cfg(test)]
mod reindex {
    use crate::ctab::{CrossingBond, SubstructureKind, Superatom};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let mut ctab = ConnectionTable::default();

        ctab.reindex().unwrap();

        assert_eq!(
            ctab,
            ConnectionTable {
                ..Default::default()
            }
        )
    }

    #[test]
    fn one_atom() {
        let mut ctab = ConnectionTable {
            atoms: vec![Atom::any(42, 0., 0.).unwrap()],
            ..Default::default()
        };

        ctab.reindex().unwrap();

        assert_eq!(
            ctab,
            ConnectionTable {
                atoms: vec![Atom {
                    index: "1".try_into().unwrap(),
                    ..Default::default()
                }],
                ..Default::default()
            }
        )
    }

    #[test]
    fn duplicate_atom() {
        let mut ctab = ConnectionTable {
            atoms: vec![Atom::any(1, 0., 0.).unwrap(), Atom::any(1, 0., 0.).unwrap()],
            ..Default::default()
        };

        assert_eq!(ctab.reindex(), Err(Error::DuplicateAtom))
    }

    #[test]
    fn bond_missing_source() {
        let mut ctab = ConnectionTable {
            bonds: vec![Bond::single(1, 1, 2).unwrap()],
            ..Default::default()
        };

        assert_eq!(ctab.reindex(), Err(Error::MissingAtom))
    }

    #[test]
    fn bond_missing_target() {
        let mut ctab = ConnectionTable {
            atoms: vec![Atom::any(1, 0., 0.).unwrap()],
            bonds: vec![Bond::single(1, 1, 2).unwrap()],
            ..Default::default()
        };

        assert_eq!(ctab.reindex(), Err(Error::MissingAtom))
    }

    #[test]
    fn duplicate_bond() {
        let mut ctab = ConnectionTable {
            atoms: vec![
                Atom::any(42, 0., 0.).unwrap(),
                Atom::any(13, 0., 0.).unwrap(),
            ],
            bonds: vec![
                Bond::single(69, 13, 42).unwrap(),
                Bond::single(69, 13, 42).unwrap(),
            ],
            ..Default::default()
        };

        assert_eq!(ctab.reindex(), Err(Error::DuplicateBond))
    }

    #[test]
    fn bond() {
        let mut ctab = ConnectionTable {
            atoms: vec![
                Atom::any(42, 0., 0.).unwrap(),
                Atom::any(13, 0., 0.).unwrap(),
            ],
            bonds: vec![Bond::single(69, 13, 42).unwrap()],
            ..Default::default()
        };

        ctab.reindex().unwrap();

        assert_eq!(
            ctab,
            ConnectionTable {
                atoms: vec![Atom::any(1, 0., 0.).unwrap(), Atom::any(2, 0., 0.).unwrap(),],
                bonds: vec![Bond::single(1, 2, 1).unwrap()],
                ..Default::default()
            }
        )
    }

    #[test]
    fn collection_absolute_stereo_missing_atom() {
        let mut ctab = ConnectionTable {
            collections: vec![Collection::AbsoluteStereo(vec!["42".try_into().unwrap()])],
            ..Default::default()
        };

        assert_eq!(ctab.reindex(), Err(Error::MissingAtom))
    }

    #[test]
    fn collection_absolute_stereo() {
        let mut ctab = ConnectionTable {
            atoms: vec![Atom::any(42, 0., 0.).unwrap()],
            collections: vec![Collection::AbsoluteStereo(vec!["42".try_into().unwrap()])],
            ..Default::default()
        };

        ctab.reindex().unwrap();

        assert_eq!(
            ctab,
            ConnectionTable {
                atoms: vec![Atom::any(1, 0., 0.).unwrap()],
                collections: vec![Collection::AbsoluteStereo(vec!["1".try_into().unwrap()])],
                ..Default::default()
            }
        )
    }

    #[test]
    fn collection_relative_stereo() {
        let mut ctab = ConnectionTable {
            atoms: vec![Atom::any(42, 0., 0.).unwrap()],
            collections: vec![Collection::RelativeStereo(
                "1".try_into().unwrap(),
                vec!["42".try_into().unwrap()],
            )],
            ..Default::default()
        };

        ctab.reindex().unwrap();

        assert_eq!(
            ctab,
            ConnectionTable {
                atoms: vec![Atom::any(1, 0., 0.).unwrap()],
                collections: vec![Collection::RelativeStereo(
                    "1".try_into().unwrap(),
                    vec!["1".try_into().unwrap()]
                )],
                ..Default::default()
            }
        )
    }

    #[test]
    fn collection_racemic_stereo() {
        let mut ctab = ConnectionTable {
            atoms: vec![Atom::any(42, 0., 0.).unwrap()],
            collections: vec![Collection::RacemicStereo(
                "1".try_into().unwrap(),
                vec!["42".try_into().unwrap()],
            )],
            ..Default::default()
        };

        ctab.reindex().unwrap();

        assert_eq!(
            ctab,
            ConnectionTable {
                atoms: vec![Atom::any(1, 0., 0.).unwrap()],
                collections: vec![Collection::RacemicStereo(
                    "1".try_into().unwrap(),
                    vec!["1".try_into().unwrap()]
                )],
                ..Default::default()
            }
        )
    }

    #[test]
    fn substructure_missing_atom() {
        let mut ctab = ConnectionTable {
            substructures: vec![Substructure {
                index: "1".try_into().unwrap(),
                atoms: vec!["42".try_into().unwrap()],
                kind: SubstructureKind::Superatom(Superatom {
                    ..Default::default()
                }),
            }],
            ..Default::default()
        };

        assert_eq!(ctab.reindex(), Err(Error::MissingAtom))
    }

    #[test]
    fn substructure_missing_crossing_bond() {
        let mut ctab = ConnectionTable {
            atoms: vec![Atom::any(1, 0., 0.).unwrap(), Atom::any(2, 0., 0.).unwrap()],
            bonds: vec![Bond::single(22, 1, 2).unwrap()],
            substructures: vec![Substructure {
                atoms: vec!["1".try_into().unwrap(), "2".try_into().unwrap()],
                kind: SubstructureKind::Superatom(Superatom {
                    crossing_bonds: vec![CrossingBond::new(13, 0., 0.).unwrap()],
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(ctab.reindex(), Err(Error::MissingBond))
    }

    #[test]
    fn atoms_and_bond_and_substructure() {
        let mut ctab = ConnectionTable {
            atoms: vec![
                Atom::any(42, 0., 0.).unwrap(),
                Atom::any(13, 0., 0.).unwrap(),
            ],
            bonds: vec![Bond::single(2, 13, 42).unwrap()],
            substructures: vec![Substructure {
                index: "2".try_into().unwrap(),
                atoms: vec!["13".try_into().unwrap(), "42".try_into().unwrap()],
                kind: SubstructureKind::Superatom(Superatom {
                    crossing_bonds: vec![CrossingBond::new(2, 0., 0.).unwrap()],
                    ..Default::default()
                }),
            }],
            ..Default::default()
        };

        ctab.reindex().unwrap();

        assert_eq!(
            ctab,
            ConnectionTable {
                atoms: vec![Atom::any(1, 0., 0.).unwrap(), Atom::any(2, 0., 0.).unwrap(),],
                bonds: vec![Bond::single(1, 2, 1).unwrap(),],
                substructures: vec![Substructure {
                    index: "2".try_into().unwrap(),
                    atoms: vec!["2".try_into().unwrap(), "1".try_into().unwrap()],
                    kind: SubstructureKind::Superatom(Superatom {
                        crossing_bonds: vec![CrossingBond::new(1, 0., 0.).unwrap(),],
                        ..Default::default()
                    }),
                }],
                ..Default::default()
            }
        )
    }
}
