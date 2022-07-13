use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum HydrogenDisplay {
    HBond1,
    HBond2,
}

impl fmt::Display for HydrogenDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::HBond1 => "HBOND1",
                Self::HBond2 => "HBOND2",
            }
        )
    }
}
