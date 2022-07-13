use std::fmt;

use super::Superatom;

#[derive(Debug, PartialEq, Clone)]
pub enum SubstructureKind {
    Superatom(Superatom),
}

impl Default for SubstructureKind {
    fn default() -> Self {
        Self::Superatom(Superatom::default())
    }
}

impl fmt::Display for SubstructureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Superatom(superatom) => superatom.fmt(f),
        }
    }
}
