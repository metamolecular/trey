use super::{Comment, DimensionalCode, Name, Timestamp};

#[derive(Debug, PartialEq)]
pub struct Header {
    pub name: Name,
    pub initials: [char; 2],
    pub program: [char; 8],
    pub timestamp: Timestamp,
    pub dimensional_code: DimensionalCode,
    pub scaling_factors: [char; 12],
    pub energy: [char; 12],
    pub registry_number: [char; 6],
    pub comments: Comment,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            name: Default::default(),
            initials: [' '; 2],
            program: ['M', 'M', ' ', 'V', '3', 'K', ' ', ' '],
            timestamp: Default::default(),
            dimensional_code: Default::default(),
            scaling_factors: [' '; 12],
            energy: [' '; 12],
            registry_number: [' '; 6],
            comments: Default::default(),
        }
    }
}
