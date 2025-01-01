use winnow_datetime::{Date, Offset, Time};

pub mod date;
pub mod offset;
pub mod time;

use winnow::error::{ContextError, ErrMode};
#[derive(Clone, Debug, PartialEq)]
pub struct FormatAssertion<T> {
    pub format: String,
    pub input: String,
    pub expected: Result<T, ErrMode<ContextError>>,
}

pub trait FormatAssertionBuilder<T> {
    fn base_assertions(&self) -> Vec<FormatAssertion<T>>;
    fn assertions(&self) -> Vec<FormatAssertion<T>>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormatCoverage<T> {
    pub format: String,
    pub exception: Result<Option<T>, ErrMode<ContextError>>,
    pub complete: bool,
}

pub trait FormatCoverageBuilder<T> {
    fn base_coverage(&self) -> Vec<FormatCoverage<T>>;
    fn coverage(&self) -> Vec<FormatCoverage<T>>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct DateCoverage {
    pub coverage: Vec<FormatCoverage<Date>>,
}

impl FormatCoverageBuilder<Date> for DateCoverage {
    fn base_coverage(&self) -> Vec<FormatCoverage<Date>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Date>> {
        vec![]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeCoverage {
    pub coverage: Vec<FormatCoverage<Time>>,
    pub separators: Vec<Option<&'static str>>,
    pub timezone_coverage: OffsetCoverage,
}

impl FormatCoverageBuilder<Time> for TimeCoverage {
    fn base_coverage(&self) -> Vec<FormatCoverage<Time>> {
        self.coverage
            .clone()
            .iter()
            .filter(|c| c.complete)
            .cloned()
            .collect()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Time>> {
        let mut acc = vec![];

        for s in self.separators.iter() {
            for t in self.coverage.clone() {
                for tz in self.timezone_coverage.coverage.iter() {
                    let format = format!("{}{}{}", t.format, s.unwrap_or(""), tz.format);
                    let exception = match (t.exception.clone(), tz.exception.clone()) {
                        (Ok(None), Ok(None)) => Ok(None),
                        (Ok(Some(t)), Ok(Some(tz))) => {
                            Ok(Some(t.set_tz(Some((tz.offset_hours, tz.offset_minutes)))))
                        }
                        (Err(e), _) => Err(e),
                        (_, Err(e)) => Err(e),
                        _ => panic!("Invalid exception combination"),
                    };

                    acc.push(FormatCoverage {
                        format,
                        exception,
                        complete: true,
                    });
                }
            }
        }

        acc
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCoverage {
    pub coverage: Vec<FormatCoverage<Offset>>,
}

impl FormatCoverageBuilder<Offset> for OffsetCoverage {
    fn base_coverage(&self) -> Vec<FormatCoverage<Offset>> {
        self.coverage.clone()
    }

    fn coverage(&self) -> Vec<FormatCoverage<Offset>> {
        vec![]
    }
}

#[macro_export]
macro_rules! generate_test_suite {
    ($suite_name: ident, $format: ident, $piece: ident, $piece_type:ty, $coverage: ident, $parser: ident) => {
        test_suite! {
            name $suite_name;

            use winnow_datetime_assert::$piece::assertions;
            use winnow_datetime_assert::$coverage;
            use winnow_datetime_assert::{FormatAssertion, FormatAssertionBuilder, FormatCoverage, FormatCoverageBuilder};
            use winnow::combinator::{eof, terminated};
            use winnow::Parser;

            fixture format_checks(assertion: crate::FormatAssertion<$piece_type>) -> crate::FormatAssertion<$piece_type> {
                params {
                    assertions().assertions().into_iter()
                }
                setup(&mut self) {
                    self.assertion.clone()
                }
            }

            // Iterate through the formats and create tests for each
            test format_assertions(format_checks) {
                let mut input = format_checks.params.assertion.input.clone();

                let result = terminated($format::parsers::$parser, eof).parse_next(&mut input.as_bytes());

                let base_coverage = $format::$piece::coverage().base_coverage();
                let coverage = $format::$piece::coverage().coverage();

                let mut coverage_iter = base_coverage.iter().chain(coverage.iter());

                if coverage_iter.any(|c| c.format == format_checks.params.assertion.format) {
                    assert_eq!(result, format_checks.params.assertion.expected);
                }
                else {
                    assert!(result.is_err(), "Uncovered format {} ({}) should not parse", format_checks.params.assertion.format, format_checks.params.assertion.input);
                }

            }
        }
   }
}
