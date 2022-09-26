use super::Error;

pub fn fortran_float(
    chars: std::slice::Iter<'_, char>,
    decimal_places: usize,
) -> Result<f32, Error> {
    let mut buffer = String::new();
    let chars = chars.enumerate().peekable();
    let dot = chars.len() - decimal_places - 1;

    for (i, char) in chars {
        if char == &' ' {
            if buffer.is_empty() {
                continue;
            } else {
                return Err(Error::InvalidCharacter(i));
            }
        } else if i == dot {
            if char == &'.' {
                buffer.push(*char)
            } else {
                return Err(Error::InvalidCharacter(i));
            }
        } else if char == &'+' {
            if buffer.is_empty() {
                buffer.push(*char)
            } else {
                return Err(Error::InvalidCharacter(i));
            }
        } else if char == &'-' {
            if buffer.is_empty() {
                buffer.push(*char)
            } else {
                return Err(Error::InvalidCharacter(i));
            }
        } else if char.is_numeric() {
            buffer.push(*char)
        } else {
            return Err(Error::InvalidCharacter(i));
        }
    }
    println!("{:?}", buffer);
    Ok(buffer.parse::<f32>().expect("parse f32"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn alpha() {
        assert_eq!(
            fortran_float(['a', '.', '3'].iter(), 1),
            Err(Error::InvalidCharacter(0))
        )
    }

    #[test]
    fn no_decimal() {
        assert_eq!(
            fortran_float(['1', '2', '3'].iter(), 1),
            Err(Error::InvalidCharacter(1))
        )
    }

    #[test]
    fn double_decimal() {
        assert_eq!(
            fortran_float(['1', '.', '.', '3'].iter(), 2),
            Err(Error::InvalidCharacter(2))
        )
    }

    #[test]
    fn trailing_space() {
        assert_eq!(
            fortran_float(['1', '.', '2', '3', ' '].iter(), 3),
            Err(Error::InvalidCharacter(4))
        )
    }

    #[test]
    fn internal_space() {
        assert_eq!(
            fortran_float(['1', ' ', '.', '2', '3'].iter(), 3),
            Err(Error::InvalidCharacter(1))
        )
    }

    #[test]
    fn internal_plus() {
        assert_eq!(
            fortran_float(['1', '+', '.', '2', '3'].iter(), 2),
            Err(Error::InvalidCharacter(1))
        )
    }

    #[test]
    fn internal_minus() {
        assert_eq!(
            fortran_float(['1', '-', '.', '2', '3'].iter(), 2),
            Err(Error::InvalidCharacter(1))
        )
    }

    #[test]
    fn leading_space() {
        assert_eq!(fortran_float([' ', '1', '.', '2', '3'].iter(), 2), Ok(1.23))
    }

    #[test]
    fn full() {
        assert_eq!(fortran_float(['1', '.', '2', '3'].iter(), 2), Ok(1.23))
    }

    #[test]
    fn leading_plus() {
        assert_eq!(fortran_float(['+', '1', '.', '2', '3'].iter(), 2), Ok(1.23))
    }

    #[test]
    fn leading_minus() {
        assert_eq!(
            fortran_float(['-', '1', '.', '2', '3'].iter(), 2),
            Ok(-1.23)
        )
    }
}
