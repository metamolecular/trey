use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum AttachmentPoint {
    First,
    Second,
    Both,
}

impl Display for AttachmentPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::First => "1",
                Self::Second => "2",
                Self::Both => "-1",
            }
        )
    }
}
