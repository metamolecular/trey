use std::fmt::Display;

use super::Element;

#[derive(Debug, PartialEq, Clone)]
pub struct ElementList {
    pub not: bool,
    pub elements: Vec<Element>,
}

impl Display for ElementList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}]",
            if self.not {
                "NOT".to_string()
            } else {
                "".to_string()
            },
            self.elements
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
