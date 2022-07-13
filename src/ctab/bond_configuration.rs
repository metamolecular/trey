use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum BondConfiguration {
    Up,
    Either,
    Down,
}

impl fmt::Display for BondConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => "1",
                Self::Either => "2",
                Self::Down => "3",
            }
        )
    }
}
