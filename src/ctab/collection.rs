use std::fmt;

use super::Index;

#[derive(Debug, PartialEq, Clone)]
pub enum Collection {
    AbsoluteStereo(Vec<Index>),
    RacemicStereo(Index, Vec<Index>),
    RelativeStereo(Index, Vec<Index>),
}

impl Collection {
    pub fn contains(&self, index: &Index) -> bool {
        match self {
            Collection::AbsoluteStereo(indexes) => indexes.contains(index),
            Collection::RacemicStereo(_, indexes) => indexes.contains(index),
            Collection::RelativeStereo(_, indexes) => indexes.contains(index),
        }
    }
}

impl Collection {
    pub fn merge(&mut self, other: &mut Collection) -> bool {
        match self {
            Self::AbsoluteStereo(self_indexes) => match other {
                Self::AbsoluteStereo(other_indexes) => {
                    self_indexes.append(other_indexes);

                    true
                }
                _ => false,
            },
            Self::RacemicStereo(self_index, self_indexes) => match other {
                Self::RacemicStereo(other_index, other_indexes) => {
                    if self_index == other_index {
                        self_indexes.append(other_indexes);

                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            Self::RelativeStereo(self_index, self_indexes) => match other {
                Self::RelativeStereo(other_index, other_indexes) => {
                    if self_index == other_index {
                        self_indexes.append(other_indexes);

                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
        }
    }
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Collection::AbsoluteStereo(indexes) =>
                    format!("MDLV30/STEABS ATOMS={}", write_indexes(indexes)),
                Collection::RacemicStereo(index, indexes) =>
                    format!("MDLV30/STERAC{} ATOMS={}", index, write_indexes(indexes)),
                Collection::RelativeStereo(index, indexes) =>
                    format!("MDLV30/STEREL{} ATOMS={}", index, write_indexes(indexes)),
            }
        )
    }
}

fn write_indexes(indexes: &Vec<Index>) -> String {
    if indexes.is_empty() {
        "(0)".to_string()
    } else {
        format!(
            "({} {})",
            indexes.len(),
            indexes
                .iter()
                .map(|index| index.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod to_string {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn absolute_stereo() {
        let collection = Collection::AbsoluteStereo(vec![
            Index::try_from("13").unwrap(),
            Index::try_from("42").unwrap(),
        ]);

        assert_eq!(collection.to_string(), "MDLV30/STEABS ATOMS=(2 13 42)")
    }

    #[test]
    fn racemic_stereo() {
        let collection = Collection::RacemicStereo(
            Index::try_from("1").unwrap(),
            vec![
                Index::try_from("13").unwrap(),
                Index::try_from("42").unwrap(),
            ],
        );

        assert_eq!(collection.to_string(), "MDLV30/STERAC1 ATOMS=(2 13 42)")
    }

    #[test]
    fn relative_stereo() {
        let collection = Collection::RelativeStereo(
            Index::try_from("2").unwrap(),
            vec![
                Index::try_from("13").unwrap(),
                Index::try_from("42").unwrap(),
            ],
        );

        assert_eq!(collection.to_string(), "MDLV30/STEREL2 ATOMS=(2 13 42)")
    }
}

#[cfg(test)]
mod merge {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn absolute_racemic() {
        let mut target = Collection::AbsoluteStereo(vec!["1".try_into().unwrap()]);
        let mut source =
            Collection::RacemicStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()]);

        assert_eq!(target.merge(&mut source), false);
        assert_eq!(
            target,
            Collection::AbsoluteStereo(vec!["1".try_into().unwrap()])
        )
    }

    #[test]
    fn absolute_absolute() {
        let mut target = Collection::AbsoluteStereo(vec!["1".try_into().unwrap()]);
        let mut source = Collection::AbsoluteStereo(vec!["2".try_into().unwrap()]);

        assert_eq!(target.merge(&mut source), true);
        assert_eq!(
            target,
            Collection::AbsoluteStereo(vec!["1".try_into().unwrap(), "2".try_into().unwrap(),])
        )
    }

    #[test]
    fn racemic_absolute() {
        let mut target =
            Collection::RacemicStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()]);
        let mut source = Collection::AbsoluteStereo(vec!["2".try_into().unwrap()]);

        assert_eq!(target.merge(&mut source), false);
        assert_eq!(
            target,
            Collection::RacemicStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()])
        )
    }

    #[test]
    fn racemic_racemic_different_group() {
        let mut target =
            Collection::RacemicStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()]);
        let mut source =
            Collection::RacemicStereo("2".try_into().unwrap(), vec!["2".try_into().unwrap()]);

        assert_eq!(target.merge(&mut source), false);
        assert_eq!(
            target,
            Collection::RacemicStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()])
        )
    }

    #[test]
    fn racemic_racemic_same_group() {
        let mut target =
            Collection::RacemicStereo("1".try_into().unwrap(), vec!["1".try_into().unwrap()]);
        let mut source =
            Collection::RacemicStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()]);

        assert_eq!(target.merge(&mut source), true);
        assert_eq!(
            target,
            Collection::RacemicStereo(
                "1".try_into().unwrap(),
                vec!["1".try_into().unwrap(), "2".try_into().unwrap()]
            )
        )
    }

    #[test]
    fn relative_absolute() {
        let mut target =
            Collection::RelativeStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()]);
        let mut source = Collection::AbsoluteStereo(vec!["2".try_into().unwrap()]);

        assert_eq!(target.merge(&mut source), false);
        assert_eq!(
            target,
            Collection::RelativeStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()])
        )
    }

    #[test]
    fn relative_relative_different_group() {
        let mut target =
            Collection::RelativeStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()]);
        let mut source =
            Collection::RelativeStereo("2".try_into().unwrap(), vec!["2".try_into().unwrap()]);

        assert_eq!(target.merge(&mut source), false);
        assert_eq!(
            target,
            Collection::RelativeStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()])
        )
    }

    #[test]
    fn relative_relative_same_group() {
        let mut target =
            Collection::RelativeStereo("1".try_into().unwrap(), vec!["1".try_into().unwrap()]);
        let mut source =
            Collection::RelativeStereo("1".try_into().unwrap(), vec!["2".try_into().unwrap()]);

        assert_eq!(target.merge(&mut source), true);
        assert_eq!(
            target,
            Collection::RelativeStereo(
                "1".try_into().unwrap(),
                vec!["1".try_into().unwrap(), "2".try_into().unwrap()]
            )
        )
    }
}

#[cfg(test)]
mod contains {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty_absolute() {
        let collection = Collection::AbsoluteStereo(vec![]);
        let index = "1".try_into().unwrap();

        assert_eq!(collection.contains(&index), false)
    }

    #[test]
    fn in_absolute() {
        let index: Index = "1".try_into().unwrap();
        let collection = Collection::AbsoluteStereo(vec![index.clone()]);

        assert_eq!(collection.contains(&index), true)
    }

    #[test]
    fn empty_racemic() {
        let collection = Collection::RacemicStereo("1".try_into().unwrap(), vec![]);
        let index = "1".try_into().unwrap();

        assert_eq!(collection.contains(&index), false)
    }

    #[test]
    fn in_racemic() {
        let index: Index = "1".try_into().unwrap();
        let collection = Collection::RacemicStereo("42".try_into().unwrap(), vec![index.clone()]);

        assert_eq!(collection.contains(&index), true)
    }

    #[test]
    fn empty_relative() {
        let collection = Collection::RelativeStereo("1".try_into().unwrap(), vec![]);
        let index = "1".try_into().unwrap();

        assert_eq!(collection.contains(&index), false)
    }

    #[test]
    fn in_relative() {
        let index: Index = "1".try_into().unwrap();
        let collection = Collection::RelativeStereo("42".try_into().unwrap(), vec![index.clone()]);

        assert_eq!(collection.contains(&index), true)
    }
}
