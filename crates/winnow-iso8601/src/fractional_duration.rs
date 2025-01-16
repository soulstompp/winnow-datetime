use crate::parsers;
use alloc::string::String;
use winnow_datetime::types::Duration;
use winnow_datetime::FractionalDuration;
use winnow_datetime_assert::{DurationCoverage, FormatCoverage};

/// Parses a duration string similiar to duration but allows for decimal places.
pub fn parse_duration(mut i: &str) -> Result<Duration, String> {
    match parsers::duration(&mut i) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Failed to parse duration {}: {}", i, e)),
    }
}

/// let duration = winnow_iso8601::parse_fractional_duration("P1,5Y2M3DT4,5H5M6S").unwrap();
/// let duration = winnow_iso8601::parse_fractional_duration("P1,5W").unwrap();
pub fn parse_fractional_duration(mut i: &str) -> Result<FractionalDuration, String> {
    match parsers::fractional_duration(&mut i) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Failed to parse duration {}: {}", i, e)),
    }
}

/// Provides coverage of formats parser is expected to handle.
#[cfg(feature = "testing")]
pub fn coverage() -> DurationCoverage {
    DurationCoverage {
        coverage: vec![
            // Format
            // P1Y
            FormatCoverage {
                format: "P1Y".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1,5Y
            FormatCoverage {
                format: "P1,5Y".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1.5Y
            FormatCoverage {
                format: "P1.5Y".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1M
            FormatCoverage {
                format: "P1M".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1W
            FormatCoverage {
                format: "P1W".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1D
            FormatCoverage {
                format: "P1D".into(),
                exception: Ok(None),
                complete: true,
            },
            // PT1H
            FormatCoverage {
                format: "PT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1H
            FormatCoverage {
                format: "P1H".into(),
                exception: Ok(None),
                complete: true,
            },
            // PT1M
            FormatCoverage {
                format: "PT1M".into(),
                exception: Ok(None),
                complete: true,
            },
            // PT1S
            FormatCoverage {
                format: "PT1S".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1S
            FormatCoverage {
                format: "P1S".into(),
                exception: Ok(None),
                complete: true,
            },
            // PT1,5S
            FormatCoverage {
                format: "PT1,5S".into(),
                exception: Ok(None),
                complete: true,
            },
            // PT1.5S
            FormatCoverage {
                format: "PT1.5S".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1Y1M
            FormatCoverage {
                format: "P1Y1M".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1Y1D
            FormatCoverage {
                format: "P1Y1D".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1Y1M1D
            FormatCoverage {
                format: "P1Y1M1D".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1Y1M1DT1H1M1S
            FormatCoverage {
                format: "P1Y1M1DT1H1M1S".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1DT1H
            FormatCoverage {
                format: "P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1MT1M
            FormatCoverage {
                format: "P1MT1M".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1DT1M
            FormatCoverage {
                format: "P1DT1M".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1.5W
            FormatCoverage {
                format: "P1.5W".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1,5W
            FormatCoverage {
                format: "P1,5W".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1DT1.000S
            FormatCoverage {
                format: "P1DT1.000S".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1DT1.00000S
            FormatCoverage {
                format: "P1DT1.00000S".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1DT1H1M1.1S
            FormatCoverage {
                format: "P1DT1H1M1.1S".into(),
                exception: Ok(None),
                complete: true,
            },
            // P1H1M1.1S
            FormatCoverage {
                format: "P1H1M1.1S".into(),
                exception: Ok(None),
                complete: true,
            },
        ],
    }
}
