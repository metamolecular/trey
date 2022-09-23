use crate::ctab::Rgroup;

use super::connection_table;

pub fn rgroup(rgroup: &Rgroup) -> Vec<String> {
    let mut result = Vec::new();

    result.push(format!("M  V30 BEGIN RGROUP {}", &rgroup.number));

    for ctab in rgroup.connection_tables.iter() {
        result.append(&mut connection_table(ctab))
    }

    result.push(format!("M  V30 END RGROUP"));

    result
}

#[cfg(test)]
mod tests {
    use crate::ctab::{Atom, ConnectionTable};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        let group = Rgroup {
            number: "13".try_into().unwrap(),
            connection_tables: vec![
                ConnectionTable {
                    atoms: vec![Atom::any(42, 1., 0.).unwrap()],
                    ..Default::default()
                },
                ConnectionTable {
                    atoms: vec![Atom::any(89, 1., 0.).unwrap()],
                    ..Default::default()
                },
            ],
        };

        assert_eq!(
            rgroup(&group),
            vec![
                "M  V30 BEGIN RGROUP 13",
                "M  V30 BEGIN CTAB",
                "M  V30 COUNTS 1 0 0 0 0",
                "M  V30 BEGIN ATOM",
                "M  V30 42 * 1 0 0 0",
                "M  V30 END ATOM",
                "M  V30 END CTAB",
                "M  V30 BEGIN CTAB",
                "M  V30 COUNTS 1 0 0 0 0",
                "M  V30 BEGIN ATOM",
                "M  V30 89 * 1 0 0 0",
                "M  V30 END ATOM",
                "M  V30 END CTAB",
                "M  V30 END RGROUP"
            ]
        )
    }
}
