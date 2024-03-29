#[derive(Debug, PartialEq)]
pub enum Error {
    StringTooLong,
    ReservedTag,
    InvalidCharacter(usize),
}
