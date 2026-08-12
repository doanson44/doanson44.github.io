#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimestampUnit {
    Seconds,
    Milliseconds,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimestampDirection {
    TimestampToDateTime,
    DateTimeToTimestamp,
}

impl TimestampUnit {
    pub fn multiplier(self) -> f64 {
        match self {
            Self::Seconds => 1_000.0,
            Self::Milliseconds => 1.0,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Seconds => "Seconds",
            Self::Milliseconds => "Milliseconds",
        }
    }
}

impl TimestampDirection {
    pub fn label(self) -> &'static str {
        match self {
            Self::TimestampToDateTime => "Unix Timestamp → Date/Time",
            Self::DateTimeToTimestamp => "Date/Time → Unix Timestamp",
        }
    }
}

pub fn timestamp_to_millis(value: &str, unit: TimestampUnit) -> Result<f64, String> {
    let value = value
        .trim()
        .parse::<f64>()
        .map_err(|_| "Enter a valid numeric timestamp.".to_string())?;
    if !value.is_finite() {
        return Err("Timestamp must be a finite number.".into());
    }
    Ok(value * unit.multiplier())
}

pub fn millis_to_timestamp(millis: f64, unit: TimestampUnit) -> String {
    let value = millis / unit.multiplier();
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value:.3}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_seconds_to_milliseconds() {
        assert_eq!(
            timestamp_to_millis("1700000000", TimestampUnit::Seconds).unwrap(),
            1_700_000_000_000.0
        );
    }

    #[test]
    fn converts_milliseconds_to_seconds() {
        assert_eq!(
            millis_to_timestamp(1_700_000_000_000.0, TimestampUnit::Seconds),
            "1700000000"
        );
    }

    #[test]
    fn rejects_invalid_timestamp() {
        assert!(timestamp_to_millis("abc", TimestampUnit::Seconds).is_err());
    }
}
