use std::fmt;

use super::wrap;

pub fn block<T: fmt::Display>(name: &str, items: &Vec<T>) -> Vec<String> {
    let mut result = Vec::new();

    if items.is_empty() {
        return result;
    }

    result.push(format!("M  V30 BEGIN {}", name));

    for item in items {
        let line = format!("M  V30 {}", item);

        match wrap(&line) {
            Some(mut lines) => result.append(&mut lines),
            None => result.push(line),
        }
    }

    result.push(format!("M  V30 END {}", name));

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let items: Vec<String> = vec![];

        assert_eq!(block("STRING", &items), vec![String::new(); 0])
    }

    #[test]
    fn full() {
        let items = vec!["ONE".to_string(), "TWO".to_string()];

        assert_eq!(
            block("STRING", &items),
            vec![
                "M  V30 BEGIN STRING",
                "M  V30 ONE",
                "M  V30 TWO",
                "M  V30 END STRING"
            ]
        )
    }

    #[test]
    fn wrap() {
        let items = vec!["X".repeat(80)];

        assert_eq!(
            block("LONG", &items),
            vec![
                "M  V30 BEGIN LONG",
                &format!("M  V30 {}-", "X".repeat(72)),
                &format!("M  V30 {}", "X".repeat(8)),
                "M  V30 END LONG"
            ]
        )
    }
}
