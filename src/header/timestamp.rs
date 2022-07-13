use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Timestamp {
    pub month: [char; 2],
    pub day: [char; 2],
    pub year: [char; 2],
    pub hours: [char; 2],
    pub minutes: [char; 2],
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.month.iter().collect::<String>(),
            self.day.iter().collect::<String>(),
            self.year.iter().collect::<String>(),
            self.hours.iter().collect::<String>(),
            self.minutes.iter().collect::<String>(),
        )
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self {
            month: ['0', '1'],
            day: ['0', '1'],
            year: ['7', '0'],
            hours: ['0', '0'],
            minutes: ['0', '0'],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        let ts = Timestamp::default();

        assert_eq!(ts.to_string(), "0101700000")
    }

    #[test]
    fn custom() {
        let ts = Timestamp {
            month: ['0', '7'],
            day: ['2', '0'],
            year: ['6', '9'],
            hours: ['2', '0'],
            minutes: ['1', '7'],
        };

        assert_eq!(ts.to_string(), "0720692017")
    }
}
