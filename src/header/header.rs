use super::{Comment, Float12, Name, ScalingFactors};

#[derive(Debug, PartialEq, Default)]
pub struct Header {
    pub name: Name,
    pub initials: Option<[char; 2]>,
    pub program: Option<[char; 8]>,
    pub timestamp: Option<[char; 10]>,
    pub dimensional_code: Option<[char; 2]>,
    pub scaling_factors: Option<ScalingFactors>,
    pub energy: Option<Float12>,
    pub registry_number: [char; 6],
    pub comments: Comment,
}
