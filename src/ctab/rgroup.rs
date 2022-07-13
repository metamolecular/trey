use super::{ConnectionTable, Index};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Rgroup {
    pub number: Index,
    pub connection_tables: Vec<ConnectionTable>,
}
