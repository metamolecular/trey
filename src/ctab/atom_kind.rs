use std::fmt;

use super::{Element, ElementList, IndexList};

#[derive(PartialEq, Debug, Clone)]
pub enum AtomKind {
    Element(Element),
    PolymerBead,
    Any,
    Rgroup(IndexList),
    ElementList(ElementList),
}

impl Default for AtomKind {
    fn default() -> Self {
        Self::Any
    }
}

impl fmt::Display for AtomKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Any => "*",
                Self::PolymerBead => "Pol",
                Self::Element(element) => return element.fmt(f),
                Self::Rgroup(_) => "R#",
                Self::ElementList(list) => return list.fmt(f),
            }
        )
    }
}
