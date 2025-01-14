use crate::parsers;
use alloc::string::String;
use winnow_datetime::types::Duration;
use winnow_datetime_assert::{DurationCoverage, FormatCoverage};

/// Parses a duration string.
///
/// A string starts with `P` and can have one of the following formats:
///
/// * Fully-specified duration: `P1Y2M3DT4H5M6S`
/// * Duration in weekly intervals: `P1W`
/// * Fully-specified duration in [`DateTime`](`crate::DateTime`) format: `P<datetime>`
///
/// Both fully-specified formats get parsed into the YMDHMS Duration variant.
/// The weekly interval format gets parsed into the Weeks Duration variant.
///
/// The ranges for each of the individual units are not expected to exceed
/// the next largest unit.
///
/// These ranges (inclusive) are as follows:
///
/// * Year (any valid u32)
/// * Month 0 - 12
/// * Week 0 - 52
/// * Day 0 - 31
/// * Hour 0 - 24
/// * Minute 0 - 60
/// * Second 0 - 60
///
/// ## Examples
///
/// ```rust
/// let duration = winnow_iso8601::parse_duration("P1Y2M3DT4H5M6S").unwrap();
/// let duration = winnow_iso8601::parse_duration("P1W").unwrap();
/// ```
pub fn parse_duration(mut i: &str) -> Result<Duration, String> {
    match parsers::duration(&mut i) {
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
        ],
    }
}
