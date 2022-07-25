use std::fmt::Display;

use lyn::Scanner;

use super::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Decimal(String);

impl TryFrom<&str> for Decimal {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut scanner = Scanner::new(value);

        match decimal(&mut scanner) {
            Ok(entered) => {
                if entered {
                    if scanner.is_done() {
                        Ok(Self(value.into()))
                    } else {
                        Err(Error::DecimalFormat)
                    }
                } else {
                    Err(Error::DecimalFormat)
                }
            }
            Err(err) => Err(err),
        }
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Self("0".into())
    }
}

impl From<&Decimal> for f32 {
    fn from(value: &Decimal) -> Self {
        value.0.parse::<f32>().expect("f32 string")
    }
}

impl From<f32> for Decimal {
    fn from(value: f32) -> Self {
        Self(value.to_string())
    }
}

impl From<&Decimal> for f64 {
    fn from(value: &Decimal) -> Self {
        value.0.parse::<f64>().expect("f64 string")
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// Based on a formal grammar presented in:
// https://cs.stackexchange.com/questions/99865/

// <decimal> ::= <sign>?<nonnegative_decimal>
fn decimal(scanner: &mut Scanner) -> Result<bool, Error> {
    sign(scanner)?;

    nonnegative_decimal(scanner)
}

// <sign> ::= "-"" | "+"
fn sign(scanner: &mut Scanner) -> Result<bool, Error> {
    Ok(match scanner.peek() {
        Some('+' | '-') => {
            scanner.pop();

            true
        }
        _ => false,
    })
}

// <nonnegative_decimal> ::= <nonnegative_integer> <fractional_part>?
fn nonnegative_decimal(scanner: &mut Scanner) -> Result<bool, Error> {
    Ok(if nonnegative_integer(scanner)? {
        fractional_part(scanner)?;

        true
    } else {
        false
    })
}

// <nonnegative_integer> ::= <positive_integer> | <zero>
fn nonnegative_integer(scanner: &mut Scanner) -> Result<bool, Error> {
    Ok(positive_integer(scanner)? || zero(scanner)?)
}

// <fractional_part> ::= <dot> <digits>
fn fractional_part(scanner: &mut Scanner) -> Result<bool, Error> {
    if dot(scanner)? {
        if digits(scanner)? {
            Ok(true)
        } else {
            Err(Error::DecimalFormat)
        }
    } else {
        Ok(false)
    }
}

// <positive_integer> ::= <nonzero_digit> <digits>?
fn positive_integer(scanner: &mut Scanner) -> Result<bool, Error> {
    Ok(if nonzero_digit(scanner)? {
        digits(scanner)?;

        true
    } else {
        false
    })
}

// <digits> ::= <digit> <digit>*
fn digits(scanner: &mut Scanner) -> Result<bool, Error> {
    Ok(if digit(scanner)? {
        loop {
            if !digit(scanner)? {
                break true;
            }
        }
    } else {
        false
    })
}

// <digit> ::= <nonzero_digit> | <zero>
fn digit(scanner: &mut Scanner) -> Result<bool, Error> {
    Ok(nonzero_digit(scanner)? | zero(scanner)?)
}

// <nonzero_digit> ::= 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
fn nonzero_digit(scanner: &mut Scanner) -> Result<bool, Error> {
    Ok(match scanner.peek() {
        Some('1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => {
            scanner.pop();

            true
        }
        _ => false,
    })
}

// <zero> ::= "0"
fn zero(scanner: &mut Scanner) -> Result<bool, Error> {
    Ok(scanner.take(&'0'))
}

// <dot> ::= "."
fn dot(scanner: &mut Scanner) -> Result<bool, Error> {
    Ok(scanner.take(&'.'))
}

#[cfg(test)]
pub mod decimal {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn alpha() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(decimal(&mut scanner), Ok(false))
    }

    #[test]
    fn zero() {
        let mut scanner = Scanner::new("0");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }

    #[test]
    fn plus_zero() {
        let mut scanner = Scanner::new("+0");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }

    #[test]
    fn minus_zero() {
        let mut scanner = Scanner::new("-0");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }

    #[test]
    fn one() {
        let mut scanner = Scanner::new("1");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }

    #[test]
    fn plus_one() {
        let mut scanner = Scanner::new("+1");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }

    #[test]
    fn minus_one() {
        let mut scanner = Scanner::new("-1");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }

    #[test]
    fn zero_one() {
        let mut scanner = Scanner::new("01");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }

    #[test]
    fn point() {
        let mut scanner = Scanner::new(".");

        assert_eq!(decimal(&mut scanner), Ok(false))
    }

    #[test]
    fn point_zero() {
        let mut scanner = Scanner::new(".0");

        assert_eq!(decimal(&mut scanner), Ok(false))
    }

    #[test]
    fn zero_point() {
        let mut scanner = Scanner::new("0.");

        assert_eq!(decimal(&mut scanner), Err(Error::DecimalFormat))
    }

    #[test]
    fn zero_point_zero() {
        let mut scanner = Scanner::new("0.0");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }

    #[test]
    fn zero_point_one() {
        let mut scanner = Scanner::new("0.1");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }

    #[test]
    fn zero_point_zero_x() {
        let mut scanner = Scanner::new("0.0x");

        assert_eq!(decimal(&mut scanner), Ok(true))
    }
}

#[cfg(test)]
pub mod float_from_str {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn alpha() {
        assert_eq!(Decimal::try_from("abc"), Err(Error::DecimalFormat))
    }

    #[test]
    fn zero_point_zero_x() {
        assert_eq!(Decimal::try_from("0.0x"), Err(Error::DecimalFormat))
    }

    #[test]
    fn valid() {
        assert_eq!(
            Decimal::try_from("+3.14159"),
            Ok(Decimal("+3.14159".into()))
        )
    }
}

#[cfg(test)]
pub mod f32_from_float {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        let float = Decimal::try_from("1.23").unwrap();

        assert_eq!(f32::from(&float), 1.23f32)
    }
}

#[cfg(test)]
pub mod float_from_f32 {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        assert_eq!(Decimal::from(1.23f32), Decimal::try_from("1.23").unwrap())
    }
}

#[cfg(test)]
pub mod f64_from_float {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        let float = Decimal::try_from("1.23").unwrap();

        assert_eq!(f64::from(&float), 1.23)
    }
}
