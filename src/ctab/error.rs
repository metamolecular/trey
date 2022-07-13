#[derive(Debug, PartialEq)]
pub enum Error {
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
