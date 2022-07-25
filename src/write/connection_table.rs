use crate::ctab::ConnectionTable;

use super::block;

pub fn connection_table(ctab: &ConnectionTable) -> Vec<String> {
    let mut result = Vec::new();

    result.push("M  V30 BEGIN CTAB".to_string());
    result.push(format!(
        "M  V30 COUNTS {} {} {} 0 {}",
        ctab.atoms.len(),
        ctab.bonds.len(),
        ctab.substructures.len(),
        if ctab.is_chiral() { "1" } else { "0" }
    ));
    result.append(&mut block("ATOM", &ctab.atoms));
    result.append(&mut block("BOND", &ctab.bonds));
    result.append(&mut block("SGROUP", &ctab.substructures));
    result.append(&mut block("COLLECTION", &ctab.collections));
    result.push("M  V30 END CTAB".to_string());

    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::ctab::{
        Atom, Bond, BondConfiguration, Collection, Coordinate, CrossingBond, Index, Substructure,
        SubstructureKind, Superatom,
    };

    #[test]
    fn empty() {
        let ctab = ConnectionTable::default();

        assert_eq!(
            connection_table(&ctab),
            [
                "M  V30 BEGIN CTAB",
                "M  V30 COUNTS 0 0 0 0 0",
                "M  V30 END CTAB",
            ]
        )
    }

    #[test]
    fn filled() {
        let ctab = ConnectionTable {
            atoms: vec![
                Atom {
                    ..Default::default()
                },
                Atom {
                    index: Index::try_from("2").unwrap(),
                    ..Default::default()
                },
            ],
            bonds: vec![Bond {
                atom1: Index::try_from("1").unwrap(),
                atom2: Index::try_from("2").unwrap(),
                configuration: Some(BondConfiguration::Up),
                ..Default::default()
            }],
            collections: vec![Collection::AbsoluteStereo(vec![
                Index::try_from("2").unwrap()
            ])],
            substructures: vec![Substructure {
                index: Index::try_from("3").unwrap(),
                atoms: vec![Index::try_from("1").unwrap()],
                kind: SubstructureKind::Superatom(Superatom {
                    label: "X".to_string(),
                    crossing_bonds: vec![CrossingBond {
                        index: Index::try_from("22").unwrap(),
                        coordinate: Coordinate::new(1.1, 2.2, 0.),
                    }],
                }),
            }],
        };

        assert_eq!(
            connection_table(&ctab),
            [
                "M  V30 BEGIN CTAB",
                "M  V30 COUNTS 2 1 1 0 1",
                "M  V30 BEGIN ATOM",
                "M  V30 1 * 0 0 0 0",
                "M  V30 2 * 0 0 0 0",
                "M  V30 END ATOM",
                "M  V30 BEGIN BOND",
                "M  V30 1 1 1 2 CFG=1",
                "M  V30 END BOND",
                "M  V30 BEGIN SGROUP",
                "M  V30 3 SUP 0 ATOMS=(1 1) XBONDS=(1 22) CSTATE=(4 22 1.1 2.2 0) LABEL=X",
                "M  V30 END SGROUP",
                "M  V30 BEGIN COLLECTION",
                "M  V30 MDLV30/STEABS ATOMS=(1 2)",
                "M  V30 END COLLECTION",
                "M  V30 END CTAB",
            ]
        )
    }
}
