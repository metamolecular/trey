#[derive(Debug, PartialEq)]
pub enum Error {
    DecimalFormat,
    IdFormat,
    InvalidCharge,
    InvalidElement,
    InvalidValence,
    MissingAtom,
    DuplicateAtom,
    DuplicateBond,
    MissingBond,
    DuplicateRgroup,
    MissingRgroup,
}
