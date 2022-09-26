use super::Error;

pub fn fortran_int(chars: std::slice::Iter<'_, char>) -> Result<usize, Error> {
    let mut buffer = String::new();

    for (i, char) in chars.enumerate() {
        if char == &' ' {
            if buffer.is_empty() {
                continue;
            } else {
                return Err(Error::InvalidCharacter(i));
            }
        } else if char.is_numeric() {
            buffer.push(*char)
        } else {
            return Err(Error::InvalidCharacter(i));
        }
    }

    Ok(buffer.parse::<usize>().expect("parse usize"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn alpha() {
        assert_eq!(
            fortran_int(['1', 'a'].iter()),
            Err(Error::InvalidCharacter(1))
        )
    }

    #[test]
    fn internal_space() {
        assert_eq!(
            fortran_int(['1', ' ', '2', '3'].iter()),
            Err(Error::InvalidCharacter(1))
        )
    }

    #[test]
    fn trailing_space() {
        assert_eq!(
            fortran_int(['1', '2', '3', ' '].iter()),
            Err(Error::InvalidCharacter(3))
        )
    }

    #[test]
    fn leading_spaces() {
        assert_eq!(fortran_int([' ', '1', '2', '3'].iter()), Ok(123))
    }

    #[test]
    fn no_spaces() {
        assert_eq!(
            fortran_int(['1', '3', '3', '3', '3', '7'].iter()),
            Ok(133337)
        )
    }
}
