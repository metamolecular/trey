use crate::header::Header;

pub fn header(header: &Header) -> Vec<String> {
    let mut result = Vec::new();

    result.push(header.name.to_string());
    result.push(
        format!(
            "{}{}{}{}{}{}{}",
            header.initials.iter().collect::<String>(),
            header.program.iter().collect::<String>(),
            header.timestamp,
            header.dimensional_code,
            header.scaling_factors.iter().collect::<String>(),
            header.energy.iter().collect::<String>(),
            header.registry_number.iter().collect::<String>(),
        )
        .trim_end()
        .to_string(),
    );
    result.push(header.comments.to_string());
    result.push("  0  0  0     0  0            999 V3000".to_string());

    result
}

#[cfg(test)]
mod tests {
    use crate::header::{Comment, Name};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        let hdr = Header::default();

        assert_eq!(
            header(&hdr),
            vec![
                "",
                "  MM V3K  01017000002D",
                "",
                "  0  0  0     0  0            999 V3000",
            ]
        )
    }

    #[test]
    fn custom_name_and_comments() {
        let hdr = Header {
            name: Name::try_from("name").unwrap(),
            comments: Comment::try_from("comment...").unwrap(),
            ..Default::default()
        };

        assert_eq!(
            header(&hdr),
            vec![
                "name",
                "  MM V3K  01017000002D",
                "comment...",
                "  0  0  0     0  0            999 V3000",
            ]
        )
    }
}
