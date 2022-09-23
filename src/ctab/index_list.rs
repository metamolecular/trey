use std::{collections::HashMap, convert, fmt};

use super::{Error, Index};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct IndexList(Vec<Index>);

impl IndexList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn reindex(&mut self, map: &mut HashMap<Index, Index>) -> Result<(), Error> {
        Ok(for old in self.0.iter_mut() {
            std::mem::swap(old, map.get_mut(&old).ok_or(Error::MissingRgroup)?);
        })
    }
}

impl convert::From<Vec<Index>> for IndexList {
    fn from(list: Vec<Index>) -> Self {
        Self(list)
    }
}

impl fmt::Display for IndexList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "(0)")
        } else {
            write!(
                f,
                "({} {})",
                self.0.len(),
                self.0
                    .iter()
                    .map(|index| index.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }
}

#[cfg(test)]
mod to_string {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let list = IndexList::default();

        assert_eq!(list.to_string(), "(0)")
    }

    #[test]
    fn filled() {
        let list = IndexList::from(vec![
            Index::try_from("13").unwrap(),
            Index::try_from("42").unwrap(),
        ]);

        assert_eq!(list.to_string(), "(2 13 42)")
    }
}

#[cfg(test)]
mod reindex {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        let mut list = IndexList::default();
        let mut map = HashMap::new();

        list.reindex(&mut map).unwrap();

        assert_eq!(list, IndexList::default())
    }

    #[test]
    fn missing_rgroup() {
        let mut list = IndexList::from(vec!["1".try_into().unwrap()]);
        let mut map = HashMap::new();

        assert_eq!(list.reindex(&mut map), Err(Error::MissingRgroup))
    }

    #[test]
    fn found_rgroup() {
        let mut list = IndexList::from(vec!["3".try_into().unwrap()]);
        let mut map = [("3".try_into().unwrap(), "1".try_into().unwrap())]
            .into_iter()
            .collect::<HashMap<_, _>>();

        list.reindex(&mut map).unwrap();

        assert_eq!(list, IndexList::from(vec!["1".try_into().unwrap()]))
    }
}
