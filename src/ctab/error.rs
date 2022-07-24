#[derive(Debug, PartialEq)]
pub enum Error {
    DecimalFormat,
    IdFormat,
    InvalidCharge,
    InvalidElement,
    MissingAtom,
    DuplicateAtom,
    DuplicateBond,
    MissingBond,
    DuplicateRgroup,
    MissingRgroup,
}
