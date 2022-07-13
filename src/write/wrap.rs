pub fn wrap(line: &str) -> Option<Vec<String>> {
    if line.len() <= LINE_LIMIT {
        return None;
    }

    let mut result = Vec::new();
    let mut rest = line.to_owned();

    while rest.len() > LINE_LIMIT {
        let cut = LINE_LIMIT - 1;

        result.push(format!("{}-", &rest[..cut]));

        rest = format!("M  V30 {}", &rest[cut..]);
    }

    result.push(rest);

    Some(result)
}

const LINE_LIMIT: usize = 80;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn under_limit() {
        let line = "AAAAA";

        assert_eq!(wrap(line), None)
    }

    #[test]
    fn at_limit() {
        let line = (0..80).map(|_| "A").collect::<String>();

        assert_eq!(wrap(&line), None)
    }

    #[test]
    fn over_limit() {
        let line = (0..20).map(|_| "ABCD").collect::<String>() + "A";

        assert_eq!(
            wrap(&line),
            Some(vec![
                line[..79].to_owned() + "-",
                "M  V30 ".to_owned() + &line[79..]
            ])
        )
    }

    #[test]
    fn far_over_limit() {
        let line = (0..40).map(|_| "ABCD").collect::<String>() + "A";

        assert_eq!(
            wrap(&line),
            Some(vec![
                line[..79].to_owned() + "-",
                "M  V30 ".to_owned() + &line[79..(2 * 79 - 7)].to_owned() + "-",
                "M  V30 ".to_owned() + &line[(2 * 79 - 7)..]
            ])
        )
    }
}
