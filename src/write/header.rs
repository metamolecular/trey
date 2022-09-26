use crate::header::Header;

pub fn header(header: &Header) -> Vec<String> {
    let mut result = Vec::new();

    result.push(header.name.to_string());

    result.push(
        format!(
            "{}{}{}{}{}{}{}",
            match &header.initials {
                Some(initials) => initials.iter().collect::<String>(),
                None => "  ".into()
            },
            match &header.program {
                Some(program) => program.iter().collect::<String>(),
                None => " ".repeat(8).into()
            },
            match &header.timestamp {
                Some(timestamp) => timestamp.iter().collect::<String>(),
                None => " ".repeat(10).into()
            },
            match &header.dimensional_code {
                Some(dimensional_code) => dimensional_code.iter().collect::<String>(),
                None => "  ".into()
            },
            if let Some(scaling_factors) = &header.scaling_factors {
                scaling_factors.to_string()
            } else {
                " ".repeat(10)
            },
            match &header.energy {
                Some(energy) => energy.to_string(),
                None => " ".repeat(10).into()
            },
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
    #[ignore]
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
    #[ignore]
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
