use crate::parsers;
use alloc::string::String;
use winnow_datetime::types::Interval;
use winnow_datetime_assert::{FormatCoverage, IntervalCoverage};

/// Parses an interval string.
///
/// A string that optionally starts with `R` and contains a combination of partial date-times in the
/// following permissible formats:
///
pub fn parse_interval(mut i: &str) -> Result<Interval, String> {
    match parsers::interval(&mut i) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Failed to parse interval {}: {}", i, e)),
    }
}

/// Provides coverage of formats parser is expected to handle.
#[cfg(feature = "testing")]
pub fn coverage() -> IntervalCoverage {
    IntervalCoverage {
        coverage: vec![
            FormatCoverage {
                format: "%Y-%M-%D/P1Y".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D/P1M".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D/P1D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%V-W%W-%w/P1Y".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%V-W%W-%w/P1M".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%V-W%W-%w/P1D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%O/P1Y".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%O/P1M".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%O/P1D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D/%Y-%M-%D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%V-W%W-%w/%V-W%W-%w".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%O/%Y-%O".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1Y/%Y-%M-%D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1Y/%V-W%W-%w".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1Y/%Y-%O".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1M/%Y-%M-%D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1M/%V-W%W-%w".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1M/%Y-%O".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1D/%Y-%M-%D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1D/%V-W%W-%w".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1D/%Y-%O".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%DT%h/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%DT%h:%m/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%DT%h:%m:%s/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%DT%h:%m:%.3s/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%DT%h:%mZ/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%V-W%W-%wT%h/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%V-W%W-%wT%h:%m/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%V-W%W-%wT%h:%m:%s/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%V-W%W-%wT%h:%m:%.3s/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%V-W%W-%wT%h:%mZ/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%OT%h/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%OT%h:%m/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%OT%h:%m:%s/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%OT%h:%m:%.3s/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%OT%h:%mZ/P1DT1H".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%M-%DT%h".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%M-%DT%h:%m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%M-%DT%h:%m:%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%M-%DT%h:%m:%.3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%M-%DT%h:%mZ".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%V-W%W-%wT%h".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%V-W%W-%wT%h:%m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%V-W%W-%wT%h:%m:%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%V-W%W-%wT%h:%m:%.3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%V-W%W-%wT%h:%mZ".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%OT%h".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%OT%h:%m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%OT%h:%m:%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%OT%h:%m:%.3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "P1DT1H/%Y-%OT%h:%mZ".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y/%Y".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M/%Y-%M".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D/%Y-%M-%D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D/%M-%D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D/%D".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M/%Y-%M".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y/%Y".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%O/%Y-%O".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%O/%O".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-W%W/%Y-W%W".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-W%W-%w/%Y-W%W-%w".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-W%W/%W".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-W%W-%w/%W-%w".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-W%W-%w/%w".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D %h:%m:%s/%Y-%M-%D %h:%m:%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D %h:%m:%s/%M-%D %h:%m:%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D %h:%m:%s/%D %h:%m:%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D %h:%m/%Y-%M-%D %h:%m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Y-%M-%D %h/%Y-%M-%D %h".into(),
                exception: Ok(None),
                complete: true,
            },
        ],
    }
}
