use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum CoordinationDisplay {
    Coordination,
    Dative,
}

impl fmt::Display for CoordinationDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Coordination => "COORD",
                Self::Dative => "DATIVE",
            }
        )
    }
}
